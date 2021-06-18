use crate::{data, Result, error::Error::*};

use chrono::prelude::*;
use futures::stream::{StreamExt,TryStreamExt};
use mongodb::bson::{doc, Bson, Document, oid::ObjectId};

const DB_NAME: &str = "warp_crud"; // database name
const PEOPLE: &str = "people"; // the people collection


// Create a few consts for inserting and requesting from database
const ID: &str = "id";
const FNAME: &str = "fname";
const LNAME: &str = "lname";
const TIMESTAMP: &str = "timestamp";


pub(crate) type Client = mongodb::Client;

pub async fn ping(client: &Client) -> Result<Document> {
    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
        .map_err(|x| MongoQueryError(x))
}

pub async fn get_people(
    client: &Client,
) -> Result<Vec<data::Person>> {
    let cursor = client
        .database(DB_NAME)
        .collection::<Document>("people")
        .find(None, None)
        .await
        .map_err(|x| MongoQueryError(x))?;

    cursor
        .filter_map(|doc| )
        .try_collect()
        .await
        .map_err(|x|MongoQueryError(x))
}

pub async fn get_person(client: &Client, obj_id: &str) -> Result<Option<data::Person>> {
    let obj_id = ObjectId::parse_str(obj_id)?;

    let result = client
        .database(DB_NAME)
        .collection::<Document>(PEOPLE)
        .find(doc! { "_id": obj_id}, None)
        .await
        .map_err(|x| MongoQueryError(x))?
        .try_next()
        .await
        .map_err(|x| MongoQueryError(x))?;

    if let Some(doc) = result {
        Ok(Some(doc_to_person(&doc)?))
    } else {
        Ok(None)
    }

}

pub async fn create_person(client: &Client, person: data::PersonRequest) -> Result<String> {
    // convert the person request into a document
    let doc = doc!{
        FNAME: person.fname,
        LNAME: person.lname,
        TIMESTAMP: Utc::now()
    };

    // Send the insertion request
    let reply = client
        .database(DB_NAME)
        .collection::<Document>("people")
        .insert_one(doc, None)
        .await.map_err(|x| MongoQueryError(x))?;

    match reply.inserted_id{
        mongodb::bson::Bson::ObjectId(id) => Ok(id.to_hex()),
        _ => Err(MongoCreateError)
    }
}

fn doc_to_person(doc: &Document) -> Result<data::Person>{
    let id = doc.get_object_id("_id")?.to_hex();
    let fname = doc.get_str("fname")?.to_owned();
    let lname = doc.get_str("lname")?.to_owned();
    let timestamp = doc.get_datetime("timestamp")?;


    Ok(data::Person{
        id,
        fname,
        lname,
        timestamp: DateTime::from(*timestamp) // this needs to be converted from a reference
    })
}