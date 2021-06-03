use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use futures::stream::{StreamExt, TryStreamExt};

pub(crate) type Client = mongodb::Client;

pub async fn ping(client: &Client) -> Result<Document> {
    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
}

pub async fn get_people(client: &Client) -> Result<Vec<Document>> {
    let cursor = client
        .database("warp_rest")
        .collection::<Document>("people")
        .find(None, None).await?;

    cursor.try_collect().await
}
