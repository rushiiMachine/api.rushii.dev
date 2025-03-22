use crate::config::ORG_NAME;
use crate::logic::github_api;
use crate::logic::github_api::GithubContributor;
use log::{debug, warn};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Global contributor cache used by requests for obtaining data.
/// The write lock should not be held for an extended period of time.
static CONTRIBUTORS_CACHE: RwLock<ContributorsState> = RwLock::new(ContributorsState::Populating);

/// Contributors cache state
#[derive(Debug, Default, Clone)]
pub enum ContributorsState {
	/// Data is currently being fetched, come back later.
	#[default]
	Populating,
	/// First data fetch failed, come back later.
	PopulatingError,
	/// Data exists but is outdated, cache for only a little time.
	Outdated(Vec<Contributor>),
	/// Data exists and is still fresh, cache for full time.
	Fresh(Instant, Vec<Contributor>),
}

/// Serializable model to be sent back as an API response.
#[derive(Serialize, Debug, Clone)]
pub struct Contributor {
	/// GitHub username
	pub username: String,
	/// GitHub avatar (full size)
	#[serde(rename = "avatarUrl")]
	pub avatar_url: String,
	/// The commit count of this user across all non-private,
	/// non-forked repositories in the Aliucord organization.
	pub commits: u32,
	/// The repositories this user committed to,
	/// in decreasing order of commit count per repository.
	pub repositories: Vec<RepositoryContributor>,
}

/// Repository-specific commit count for a user.
#[derive(Serialize, Debug, Clone)]
pub struct RepositoryContributor {
	/// The repository name
	pub name: String,
	/// The commit count of this user in this repository.
	pub commits: u32,
}

/// Starts the background task to cache contributors data once a day.
pub async fn init_service() {
	const ONE_HOUR: Duration = Duration::from_secs(60 * 60);
	const ONE_DAY: Duration = Duration::from_secs(60 * 60 * 24);

	tokio::task::spawn(async {
		let mut interval = tokio::time::interval(ONE_DAY);

		loop {
			interval.tick().await; // First tick completes instantly

			match fetch_contributors().await {
				Ok(contributors) => {
					let mut guard = CONTRIBUTORS_CACHE.write()
						.unwrap_or_else(|poison| poison.into_inner());

					*guard = ContributorsState::Fresh(Instant::now(), contributors);
					CONTRIBUTORS_CACHE.clear_poison();

					interval.reset_after(ONE_DAY);
				}
				Err(err) => {
					warn!("Failed to refresh contributors, retrying in 1h: {err:?}");

					let mut guard = CONTRIBUTORS_CACHE.write()
						.unwrap_or_else(|poison| poison.into_inner());

					match std::mem::take(&mut *guard) {
						ContributorsState::Populating =>
							*guard = ContributorsState::PopulatingError,
						ContributorsState::Fresh(_, data) =>
							*guard = ContributorsState::Outdated(data),
						_ => {}
					}

					interval.reset_after(ONE_HOUR);
				}
			}
		}
	}).await.unwrap();
}

/// Clones the cached contributors list.
/// If the value is [None], then fetching contributors must have
/// either failed, or it is currently in progress.
pub async fn get_contributors() -> ContributorsState {
	CONTRIBUTORS_CACHE
		.read()
		.unwrap_or_else(|poison| poison.into_inner())
		.clone()
}

/// Tries to fetch the contributors list from GitHub from scratch.
async fn fetch_contributors() -> reqwest::Result<Vec<Contributor>> {
	// Repo name mapped to raw GitHub contributor data
	let mut contributors_flat = Vec::<(String, GithubContributor)>::new();

	debug!("Refreshing contributors cache, fetching repositories for {ORG_NAME}");
	let repos = github_api::fetch_org_repositories(ORG_NAME).await?;

	// Fetch contributor list for each repo
	for repo in repos {
		if repo.private || repo.fork || repo.name == "badges" { continue; };

		debug!("Fetching contributors for {ORG_NAME}/{0}", repo.name);
		let contributors = github_api::fetch_contributors(ORG_NAME, &*repo.name).await?
			.into_iter()
			.map(|user| (repo.name.clone(), user));

		contributors_flat.extend(contributors);
	}

	// Group by contributor name
	let mut contributors_mapped = HashMap::<String, Contributor>::new();
	for (repo, user) in contributors_flat {
		match contributors_mapped.get_mut(&user.login) {
			Some(c) => {
				c.commits += user.contributions;
				c.repositories.push(RepositoryContributor { name: repo, commits: user.contributions });
			}
			None => {
				contributors_mapped.insert(user.login.clone(), Contributor {
					username: user.login,
					avatar_url: user.avatar_url,
					commits: user.contributions,
					repositories: vec![RepositoryContributor { name: repo, commits: user.contributions }],
				});
			}
		}
	}

	// Remove bots
	contributors_mapped.remove("actions-user");
	contributors_mapped.remove("crowdin-bot");

	// Collect into a list and sort
	let mut contributors: Vec<Contributor> = contributors_mapped.into_values().collect();
	contributors.sort_by(|c1, c2| c2.commits.cmp(&c1.commits));
	contributors.iter_mut().for_each(|c|
		c.repositories.sort_by(|r1, r2| r2.commits.cmp(&r1.commits)));

	debug!("Parsed contributors data: {contributors:?}");
	Ok(contributors)
}
