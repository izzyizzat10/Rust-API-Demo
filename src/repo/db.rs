// DB Repository
use dotenv::dotenv;
use dotenv_codegen::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

use crate::models::holder_model::Holder;

pub struct DBRepo {
    holder: Collection<Holder>,
}

impl DBRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = dotenv!("MONGOURI");
        let database = dotenv!("DBNAME");

        let client = Client::with_uri_str(uri).unwrap();

        client
            .database(database)
            .run_command(doc! {"ping": 1}, None)
            .unwrap();

        println!("Database Connected!");

        let db = client.database(database);

        let holder: Collection<Holder> = db.collection("Holder");

        DBRepo { holder }
    }

    pub fn get_holders(&self) -> Result<Vec<Holder>, Error> {
        let holders = self
            .holder
            .find(None, None)
            .ok()
            .expect("Error getting holders");

        let holders = holders.map(|holder| holder.unwrap()).collect();

        Ok(holders)
    }

    pub fn create_holder(&self, new_holder: Holder) -> Result<InsertOneResult, Error> {
        let new_holder = Holder {
            id: None,
            name: new_holder.name,
            status: new_holder.status,
        };

        let new_holder = self
            .holder
            .insert_one(new_holder, None)
            .ok()
            .expect("Error creating Holder data");

        Ok(new_holder)
    }

    pub fn get_holder(&self, id: &String) -> Result<Holder, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};
        let find_holder = self
            .holder
            .find_one(filter, None)
            .ok()
            .expect("Error getting Holder data");

        Ok(find_holder.unwrap())
    }

    pub fn update_holder(
        &self,
        id: &String,
        updated_holder: Holder,
    ) -> Result<UpdateResult, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};

        let new_holder = doc! {
            "$set": {
                "id": id,
                "name": updated_holder.name,
                "status": updated_holder.status,
            },
        };

        let updated_holder = self
            .holder
            .update_one(filter, new_holder, None)
            .ok()
            .expect("Error updating holder");

        Ok(updated_holder)
    }

    pub fn delete_holder(&self, id: &String) -> Result<DeleteResult, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};

        let holder_to_delete = self
            .holder
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting holder");

        Ok(holder_to_delete)
    }
}
