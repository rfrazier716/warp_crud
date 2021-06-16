use crate::data;

use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Bson, Document};
use mongodb::error::Result;

const DB_NAME: &str = "warp_crud";

pub(crate) type Client = mongodb::Client;

pub async fn ping(client: &Client) -> Result<Document> {
    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
}

pub async fn get_people(
    client: &Client,
    filter: impl Into<Option<Document>>,
) -> Result<Vec<data::Person>> {
    let cursor = client
        .database(DB_NAME)
        .collection::<data::Person>("people")
        .find(filter, None)
        .await?;

    cursor.try_collect().await
}

pub async fn get_person(client: &Client, first_name: &str) -> Result<Option<data::Person>> {
    client
        .database(DB_NAME)
        .collection::<data::Person>("people")
        .find(doc! { "fname": first_name}, None)
        .await?
        .try_next()
        .await
}

pub async fn create_person(client: &Client, person: data::PersonRequest) -> Result<Bson> {
    // convert the person request into a document
    let mut doc = mongodb::bson::to_document(&person)?;
    //add a timestamp for the update value
    doc.insert("timestamp", Utc::now());
    let reply = client
        .database(DB_NAME)
        .collection::<Document>("people")
        .insert_one(doc, None)
        .await?;
    Ok(reply.inserted_id)
}
