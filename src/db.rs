use crate::{data, error::Error::*, Result};

use chrono::prelude::*;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::bson::{doc, oid::ObjectId, Document};

const DB_NAME: &str = "warp_crud"; // database name
const PEOPLE: &str = "people"; // the people collection

// Create a few consts for inserting and requesting from database
const ID: &str = "_id";
const FNAME: &str = "fname";
const LNAME: &str = "lname";
const TIMESTAMP: &str = "timestamp";

pub(crate) type Client = mongodb::Client;

pub async fn ping(client: &Client) -> Result<Document> {
    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
        .map_err(MongoQueryError)
}

pub async fn get_people(client: &Client) -> Result<Vec<data::Person>> {
    let cursor = client
        .database(DB_NAME)
        .collection::<Document>("people")
        .find(None, None)
        .await
        .map_err(MongoQueryError)?;

    // The cursor returns a Stream of Result<T>, so we need to map the error and then wrap the whole
    // vec in a result
    cursor
        .map(|reply| match reply {
            Ok(doc) => doc_to_person(&doc),
            Err(e) => Err(MongoQueryError(e)),
        })
        .try_collect()
        .await
}

pub async fn get_person(client: &Client, obj_id: &str) -> Result<data::Person> {
    let obj_id = ObjectId::parse_str(obj_id)?;

    let result = client
        .database(DB_NAME)
        .collection::<Document>(PEOPLE)
        .find(doc! { ID: obj_id}, None)
        .await
        .map_err(MongoQueryError)?
        .try_next()
        .await
        .map_err(MongoQueryError)?;

    if let Some(doc) = result {
        Ok(doc_to_person(&doc)?)
    } else {
        Err(NonexistentResourceError)
    }
}

pub async fn create_person(client: &Client, person: data::PersonRequest) -> Result<String> {
    // convert the person request into a document
    let doc = doc! {
        FNAME: person.fname,
        LNAME: person.lname,
        TIMESTAMP: Utc::now()
    };

    // Send the insertion request
    let reply = client
        .database(DB_NAME)
        .collection::<Document>("people")
        .insert_one(doc, None)
        .await
        .map_err(MongoQueryError)?;

    match reply.inserted_id {
        mongodb::bson::Bson::ObjectId(id) => Ok(id.to_hex()),
        _ => Err(MongoCreateError),
    }
}

pub async fn update_person(
    client: &Client,
    obj_id: &str,
    person: data::PersonRequest,
) -> Result<()> {
    // convert the object id to a mongodb id
    let obj_id = ObjectId::parse_str(obj_id)?;

    let doc = doc! { "$set": {
        FNAME: person.fname,
        LNAME: person.lname,
        TIMESTAMP: Utc::now()
        }
    };

    let filter = doc! { ID: obj_id};

    let result = client
        .database(DB_NAME)
        .collection::<Document>("people")
        .update_one(filter, doc, None)
        .await
        .map_err(MongoQueryError)?;

    // if we neither matched nor updated any document return a 404 error
    if (result.matched_count, result.modified_count) == (0,0) {
        Err(NonexistentResourceError)
    } else {
        Ok(())
    }
}


pub async fn delete_person(client: &Client, obj_id: &str) -> Result<()> {
    // convert object id to mongodb ID
    let obj_id = ObjectId::parse_str(obj_id)?;

    let filter = doc! {ID: obj_id}; // the filter for the item to delete

    let result = client
        .database(DB_NAME)
        .collection::<Document>("people")
        .delete_one(filter, None)
        .await
        .map_err(MongoQueryError)?;

    // if nothing was deleted raise a 404 error
    match result.deleted_count {
        0 => Err(NonexistentResourceError),
        _ => Ok(())
    }
}

fn doc_to_person(doc: &Document) -> Result<data::Person> {
    let id = doc.get_object_id(ID)?.to_hex(); //Fields will always need an ID
    let fname = doc.get_str(FNAME)?.to_owned();
    let lname = doc.get_str(LNAME).unwrap_or("").to_owned(); // we'll allow options without lastnames
    let timestamp = doc.get_datetime(TIMESTAMP)?; //Need to have a datetime

    Ok(data::Person {
        id,
        fname,
        lname,
        timestamp: DateTime::from(*timestamp), // this needs to be converted from a reference
    })
}
