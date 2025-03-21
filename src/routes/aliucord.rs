use crate::logic;
use crate::logic::aliucord_contributors::{Contributor, ContributorsState};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{get, routes};

#[get("/contributors")]
async fn contributors() -> Json<Vec<Contributor>> {
	match logic::aliucord_contributors::get_contributors().await {
		ContributorsState::Populating => todo!("gracefully error"),
		ContributorsState::Outdated(data) |
		ContributorsState::Fresh(data) => Json(data),
	}
}

pub fn routes() -> AdHoc {
	AdHoc::on_ignite("Aliucord API Routing", |rocket| async {
		rocket.mount("/aliucord", routes![contributors])
	})
}
