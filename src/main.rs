#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

use rocket::{get, http::Status, serde::json::Json};

mod api;
mod models;
mod repo;

use api::holder_service::{create_holder, delete_holder, get_holder, get_holders, update_holder};
use repo::db::DBRepo;

#[launch]
fn rocket() -> _ {
    let db = DBRepo::init();

    rocket::build().manage(db).mount(
        "/",
        routes![
            index,
            create_holder,
            get_holder,
            update_holder,
            delete_holder,
            get_holders
        ],
    )
}

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Rust API with Rocket and mongoDB")))
}
