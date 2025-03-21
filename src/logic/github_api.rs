use crate::config::GITHUB_TOKEN;
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder};
use rocket::serde::Deserialize;
use std::sync::LazyLock;
use std::time::Duration;

/// The HTTP client that will be used for sending GitHub API requests.
const GITHUB_HTTP: LazyLock<Client> = LazyLock::new(|| {
	let mut headers = HeaderMap::new();
	headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
	headers.insert("Authorization", format!("Bearer {0}", &*GITHUB_TOKEN).parse().unwrap());
	headers.insert("User-Agent", "api.rushii.dev (github.com/rushiiMachine/api.rushii.dev)".parse().unwrap());
	headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());

	// TODO: enable compression
	ClientBuilder::new()
		.timeout(Duration::from_secs(60))
		.default_headers(headers)
		.build()
		.unwrap()
});

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GithubContributor {
	pub login: String,
	pub avatar_url: String,
	pub contributions: u32,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GithubRepository {
	pub name: String,
	pub fork: bool,
	pub private: bool,
}

/// Fetches all contributors info for a GitHub repository.
pub async fn fetch_contributors(owner: &str, repo: &str) -> reqwest::Result<Vec<GithubContributor>> {
	let url = format!("https://api.github.com/repos/{owner}/{repo}/contributors?per_page=100");

	GITHUB_HTTP.get(url)
		.send().await?
		.error_for_status()?
		.json().await
}

/// Fetches all repositories under an organization.
pub async fn fetch_org_repositories(org: &str) -> reqwest::Result<Vec<GithubRepository>> {
	let url = format!("https://api.github.com/orgs/{org}/repos?per_page=100");

	GITHUB_HTTP.get(url)
		.send().await?
		.error_for_status()?
		.json().await
}
