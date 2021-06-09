use crate::data;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;

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
        .database("warp_rest")
        .collection::<data::Person>("people")
        .find(filter, None)
        .await?;

    cursor.try_collect().await
}

pub async fn get_person(client: &Client, first_name: &str) -> Result<Option<data::Person>> {
    client
        .database("warp_rest")
        .collection::<data::Person>("people")
        .find(doc! { "fname": first_name}, None)
        .await?
        .try_next()
        .await
}
