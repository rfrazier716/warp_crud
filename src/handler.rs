use warp::http::StatusCode;
use warp::Reply;

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
        MongoOidError(oid_error) => Ok(Box::new(
            warp::http::Response::builder()
                .status(400)
                .body(format!("{}", oid_error)),
        )),
        MongoQueryError(mongo_error) => Ok(Box::new(
            warp::http::Response::builder()
                .status(500)
                .body(format!("{}", mongo_error)),
        )),
        NonexistentResourceError => Ok(Box::new(
            warp::http::Response::builder()
                .status(404)
                .body("404: Not Found"),
        )),
        _ => Ok(Box::new(
            warp::http::Response::builder()
                .status(500)
                .body("500: Unhandled Exception"),
        )),
    }
}

pub async fn health(client: db::Client) -> Result<Box<dyn Reply>, Infallible> {
    tracing::info!("Pinging Database");
    warp_handle!(db::ping(&client).await);
    Ok(Box::new(StatusCode::OK))
}

pub mod people {

    use super::*;

    pub async fn create(
        client: db::Client,
        person: data::PersonRequest,
    ) -> Result<Box<dyn Reply>, Infallible> {
        let reply = warp_handle!(db::create_person(&client, person).await);
        Ok(Box::new(warp::reply::json(&reply)))
    }

    pub async fn read_all(client: db::Client) -> Result<Box<dyn Reply>, Infallible> {
        let reply = warp_handle!(db::get_people(&client).await);
        Ok(Box::new(warp::reply::json(&reply)))
    }

    pub async fn read_single<T>(
        client: db::Client,
        user_id: T,
    ) -> Result<Box<dyn Reply>, Infallible>
    where
        T: AsRef<str>,
    {
        let person = warp_handle!(db::get_person(&client, user_id.as_ref()).await);
        Ok(Box::new(warp::reply::json(&person)))
    }

    pub async fn update(
        client: db::Client,
        user_id: impl AsRef<str>,
        person_request: data::PersonRequest,
    ) -> Result<Box<dyn Reply>, Infallible> {
        warp_handle!(db::update_person(&client, user_id.as_ref(), person_request).await);
        Ok(Box::new(StatusCode::OK)) //return a success if the update occured
    }

    pub async fn delete(
        client: db::Client,
        user_id: impl AsRef<str>,
    ) -> Result<Box<dyn Reply>, Infallible> {
        warp_handle!(db::delete_person(&client, user_id.as_ref()).await);
        Ok(Box::new(StatusCode::OK))
    }
}
