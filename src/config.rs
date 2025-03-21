use std::str::FromStr;
use std::sync::LazyLock;

pub const ORG_NAME: &str = "Aliucord";

/// The port to bind the API server to.
pub const PORT: LazyLock<u16> = LazyLock::new(|| {
	match std::env::var("HOST") {
		Ok(var) => u16::from_str(&var).expect("invalid PORT env"),
		Err(_) => match cfg!(debug_assertions) {
			true => 8000,
			false => panic!("cannot run in production without PORT env"),
		},
	}
});

/// The GitHub API token that will be used to fetch data from the Aliucord organization.
/// If this is a PAT, then it should have access to read public repo data from the Aliucord organization.
pub const GITHUB_TOKEN: LazyLock<String> = LazyLock::new(|| {
	match std::env::var("GITHUB_TOKEN") {
		Ok(var) => var,
		Err(_) => panic!("missing GITHUB_TOKEN env"),
	}
});
