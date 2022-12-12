use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

use crate::{models::holder_model::Holder, repo::db::DBRepo};

#[post("/holder", data = "<new_holder>")]
pub fn create_holder(
    db: &State<DBRepo>,
    new_holder: Json<Holder>,
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
        return Err(Status::BadRequest);
    }

    let holder = db.get_holder(&id);

    match holder {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/holder/<id>", data = "<updated_holder>")]
pub fn update_holder(
    db: &State<DBRepo>,
    id: String,
    updated_holder: Json<Holder>,
) -> Result<Json<Holder>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let updated_holder = Holder {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: updated_holder.name.to_owned(),
        status: updated_holder.status.to_owned(),
    };

    let updated_holder = db.update_holder(&id, updated_holder);

    match updated_holder {
        Ok(response) => {
            if response.matched_count == 1 {
                let existing_holder = db.get_holder(&id);
                return match existing_holder {
                    Ok(holder) => Ok(Json(holder)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/user/<id>")]
pub fn delete_holder(db: &State<DBRepo>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let delete_holder = db.delete_holder(&id);

    match delete_holder {
        Ok(response) => {
            if response.deleted_count == 1 {
                return Ok(Json("Holder deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/holders")]
pub fn get_holders(db: &State<DBRepo>) -> Result<Json<Vec<Holder>>, Status> {
    match db.get_holders() {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(Status::InternalServerError),
    }
}
