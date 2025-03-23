mod config;
mod fairings;
mod logic;
mod routes;

use rocket::{catchers, get, launch, routes, Build, Config, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
	// TODO: launch as a fairing from Rocket, so that logger will always be initialized without delay
	tokio::spawn(async {
		tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
		logic::aliucord_contributors::init_service().await;
	});

	let figment = Config::figment()
		.merge(("port", &*config::PORT))
		.merge(("address", "0.0.0.0"))
		.merge(("ident", false));

	#[cfg(debug_assertions)]
	let figment = figment.merge(("log_level", "debug"));
	#[cfg(not(debug_assertions))]
	let figment = figment.merge(("log_level", "normal"));

	rocket::custom(figment)
		.attach(routes::aliucord::routes())
		.mount("/", routes![root])
		.register("/", catchers![
			routes::catchers::catch_500,
			routes::catchers::catch_400,
			routes::catchers::catch_404,
			routes::catchers::catch_all,
		])
}

#[get("/")]
fn root() -> &'static str {
	"read if cute <3"
}
