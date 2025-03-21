use crate::fairings::api_error::ApiError;
use crate::fairings::headers::{CacheControl, RetryAfter};
use crate::logic;
use crate::logic::aliucord_contributors::{Contributor, ContributorsState};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{get, routes, Responder};
use std::time::{Duration, Instant};

#[derive(Responder)]
enum ContributorsResponse {
	Failure(RetryAfter<ApiError>),
	Data(CacheControl<Json<Vec<Contributor>>>),
}

#[get("/contributors")]
async fn contributors() -> ContributorsResponse {
	const SOON: Duration = Duration::from_secs(10);
	const ONE_HOUR: Duration = Duration::from_secs(60 * 60);
	const ONE_DAY: Duration = Duration::from_secs(60 * 60 * 24);

	match logic::aliucord_contributors::get_contributors().await {
		ContributorsState::Populating =>
			ContributorsResponse::Failure(RetryAfter(SOON, ApiError::TemporarilyUnavailable)),
		ContributorsState::PopulatingError =>
			ContributorsResponse::Failure(RetryAfter(ONE_HOUR, ApiError::InternalServerError)),
		ContributorsState::Outdated(data) =>
			ContributorsResponse::Data(CacheControl(ONE_HOUR, Json(data))),
		ContributorsState::Fresh(time, data) =>
			ContributorsResponse::Data(CacheControl(ONE_DAY - Instant::now().duration_since(time), Json(data))),
	}
}

pub fn routes() -> AdHoc {
	AdHoc::on_ignite("Aliucord API Routing", |rocket| async {
		rocket.mount("/aliucord", routes![contributors])
	})
}
