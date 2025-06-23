mod controllers;
mod models;
mod views;
mod services;

use rocket::{launch, Rocket, Build};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> Rocket<Build> {
    println!("{:?}", rocket::Config::figment().extract_inner::<String>("template_dir"));
    rocket::build()
        .attach(Template::fairing())
        .mount("/", views::views::routes())
        .mount("/static", rocket::fs::FileServer::from("src/static"))
}
