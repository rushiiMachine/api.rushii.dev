use rocket::{get, launch, routes, Build, Config, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    let figment = Config::figment()
        .merge(("port", 8081))
        .merge(("ident", false))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment).mount("/", routes![root])
}

#[get("/")]
fn root() -> &'static str {
    "read if cute <3"
}
