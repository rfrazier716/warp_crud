use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub timestamp:  DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PersonRequest {
    pub fname: String,
    pub lname: String,
}
