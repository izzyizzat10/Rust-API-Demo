use rocket::{ http::Status, serde::json::Json, State };
use mongodb::results::InsertOneResult;

use crate::{ models::holder_model::Holder, repo::db::DBRepo };

#[post("/holder", data = "<new_holder>")]
pub fn create_holder(
        db: &State<DBRepo>, new_holder: Json<Holder>
) -> Result<Json<InsertOneResult>, Status> {
    let new_holder = Holder {
        id: None,
        name: new_holder.name.to_owned(),
        status: new_holder.status.to_owned(),
    };

    let create_holder = db.create_holder(new_holder);

    match create_holder {
        Ok(holder) => Ok(Json(holder)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/holder/<id>")]
pub fn get_holder(db: &State<DBRepo>, id: String) -> Result<Json<Holder>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest)
    }

    let result = db.get_holder(&id);

    match result {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }

}