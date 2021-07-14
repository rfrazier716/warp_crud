use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::error::Error::*;
use crate::{data, db};
use std::convert::Infallible;


macro_rules! warp_handle {
    // handles errors generated during the handler that should generate http responses
    ($result:expr) => {
        match $result {
            Ok(x) => x,
            Err(error) => return recover(error),
        }
    };
}

pub fn recover(error: crate::error::Error) -> Result<Box<dyn Reply>, Infallible> {
    match error {
        MongoOidError(oid) => Ok(Box::new(
            warp::http::Response::builder()
                .status(400)
                .body(format!("{}", oid)),
        )),
        MongoQueryError(mongo_error) => Ok(Box::new(
            warp::http::Response::builder()
                .status(500)
                .body(format!("{}", mongo_error)),
        )),
        _ => Ok(Box::new(
            warp::http::Response::builder()
                .status(500)
                .body("500: Unhandled Exception"),
        )),
    }
}

pub async fn health(client: db::Client) -> Result<impl Reply, Rejection> {
    tracing::info!("Pinging Database");
    db::ping(&client).await?;
    Ok(StatusCode::OK)
}

pub mod people {

    use super::*;

    pub async fn create(
        client: db::Client,
        person: data::PersonRequest,
    ) -> Result<impl Reply, Rejection> {
        let reply = db::create_person(&client, person).await?;
        Ok(warp::reply::json(&reply))
    }

    pub async fn read_all(client: db::Client) -> Result<impl Reply, Rejection> {
        let reply = db::get_people(&client).await?;
        Ok(warp::reply::json(&reply))
    }

    pub async fn read_single<T>(client: db::Client, user_id: T) -> Result<Box<dyn Reply>, Infallible>
    where
        T: AsRef<str>,
    {
        if let Some(person) = warp_handle!(db::get_person(&client, user_id.as_ref()).await) {
            Ok(Box::new(warp::reply::json(&person)))
        } else {
            Ok(Box::new(
                warp::http::Response::builder()
                    .status(404)
                    .body("404: Not Found"),
            ))
        }
    }

    pub async fn update<T>(
        client: db::Client,
        user_id: T,
        person_request: data::PersonRequest,
    ) -> Result<impl Reply, Rejection>
    where
        T: AsRef<str>,
    {
        db::update_person(&client, user_id.as_ref(), person_request).await?;
        Ok(StatusCode::OK) //return a success if the update occured
    }

    pub async fn delete<T>(client: db::Client, user_id: T) -> Result<impl Reply, Rejection>
    where
        T: AsRef<str>,
    {
        db::delete_person(&client, user_id.as_ref()).await?;
        Ok(StatusCode::OK)
    }
}
