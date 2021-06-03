use crate::db;
use crate::error::ServerError::DataBaseError;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

pub async fn health(client: db::Client) -> Result<impl Reply, Rejection> {
    tracing::info!("Pinging Database");
    db::ping(&client)
        .await
        .map_err(|e| DataBaseError { source: e })?;
    Ok(StatusCode::OK)
}

pub async fn get_people(client: db::Client) -> Result<impl Reply, Rejection> {
    let reply = db::get_people(&client)
        .await
        .map_err(|source| DataBaseError {source})?;
    Ok(warp::reply::json(&reply))
}