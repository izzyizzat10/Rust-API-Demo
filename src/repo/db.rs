// DB Repository
use std::env;
use dotenv::dotenv;
use mongodb::{  bson::{extjson::de::Error, oid::ObjectId, doc},
    results::InsertOneResult,
    sync::{Client, Collection}, };

use crate::models::holder_model::Holder;

pub struct DBRepo {
    col: Collection<Holder>
}

impl DBRepo {
    pub fn init() -> Self {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();

        let database = "Rust-API-Demo";

        client.database(database).run_command(doc! {"ping": 1}, None).unwrap();

        for db_name in client.list_databases(None, None).unwrap() {
            println!("Dbs: {:?}", db_name);
        }

        let db = client.database(database);
        let col: Collection<Holder> = db.collection("Holder");


        DBRepo { col }
    }

    pub fn create_holder(&self, new_holder: Holder) -> Result<InsertOneResult, Error> {
        let new_holder = Holder {
            id: None,
            name: new_holder.name,
            status: new_holder.status,
//            address_ids: new_holder.address_ids,
        };

        let holder = self.col.insert_one(new_holder, None).ok().expect("Error creating Holder data");

        Ok(holder)
    }

    pub fn get_holder(&self, id: &String) -> Result<Holder, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": obj_id};
        let find_holder = self.col.find_one(filter, None).ok().expect("Error getting Holder data");

        Ok(find_holder.unwrap())
    }
}