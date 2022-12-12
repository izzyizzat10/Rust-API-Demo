use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

//#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
//#[serde(tag = "type")]
//pub enum HolderStatus {
//    Active,
//    Inactive,
//}

#[derive(Serialize, Deserialize)]
pub struct Holder {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub status: String,
//    pub address_ids: Vec<String>,
}