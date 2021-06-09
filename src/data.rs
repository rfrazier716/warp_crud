use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub _id: ObjectId,
    pub fname: String,
    pub lname: String,
    pub timestamp: mongodb::bson::DateTime,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PersonRequest {
    pub fname: String,
    pub lname: String,
}
