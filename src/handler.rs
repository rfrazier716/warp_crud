use warp::http::StatusCode;
use warp::{Rejection, Reply};
use mongodb::bson;

use crate::error::Error::DataBaseError;
use crate::{data, db};

pub async fn health(client: db::Client) -> Result<impl Reply, Rejection> {
    tracing::info!("Pinging Database");
    db::ping(&client)
        .await
        .map_err(|e| DataBaseError { source: e })?;
    Ok(StatusCode::OK)
}

pub mod people {
    use super::*;
    use mongodb::bson::doc;
    use std::str::FromStr;

    pub async fn create(
        client: db::Client,
        person: data::PersonRequest,
    ) -> Result<impl Reply, Rejection> {
        let reply = db::create_person(&client, person)
            .await
            .map_err(|source| DataBaseError { source })?;
        Ok(warp::reply::json(&reply))
    }

    pub async fn read_all(client: db::Client) -> Result <impl Reply, Rejection> {
        let reply = db::get_people(&client)
            .await
            .map_err(|source| DataBaseError {source})?;
        Ok(warp::reply::json(&reply))
    }

    // pub async fn read_single<T>(client: db::Client, user_id: T) -> Result<impl Reply, Rejection>
    // where T: AsRef<str>
    // {
    //     //TODO!: Finish this function to accept an Object ID
    //     let user_id = bson::oid::ObjectId::from_str(user_id.as_ref()).map_err(|source| DataBaseError {source})?;
    //     let reply = db::get_person(&client, user_id)
    //         .await
    //         .map_err(|source| DataBaseError { source })?;
    //     Ok(warp::reply::json(&reply))
    // }
}
