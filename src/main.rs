mod api;
mod repo;
mod models;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

use api::holder_service::{create_holder, get_holder};
use repo::db::DBRepo;


#[launch]
fn rocket() -> _ {
    let db = DBRepo::init();

    rocket::build().manage(db).mount("/", routes![create_holder, get_holder])
}


#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Rust API with Rocket and mongoDB")))
}


