use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Person {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PersonRequest {
    pub fname: String,
    pub lname: String,
}

#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub name: String,
    pub timestamp: DateTime<Utc>,
}

pub struct Session(uuid::Uuid);
