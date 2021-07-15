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
    tracing::warn!(error = ?error, "Error occurred during span");
    match error {
        MongoOidError(oid_error) => {
            tracing::warn!("Unable to serve request, invalid Object ID");
            Ok(Box::new(
                warp::http::Response::builder()
                    .status(400)
                    .body(format!("{}", oid_error)),
            ))
        }
        MongoQueryError(mongo_error) => {
            tracing::warn!("Error Querying Database");
            Ok(Box::new(
                warp::http::Response::builder()
                    .status(500)
                    .body(format!("{}", mongo_error)),
            ))
        }
        NonexistentResourceError => {
            tracing::warn!("Requested Resource does not exist in database");
            Ok(Box::new(
                warp::http::Response::builder()
                    .status(404)
                    .body("404: Not Found"),
            ))
        }
        _ => {
            tracing::warn!("Unhandled Exception occurred");
            Ok(Box::new(
                warp::http::Response::builder()
                    .status(500)
                    .body("500: Unhandled Exception"),
            ))
        }
    }
}

pub async fn health(client: db::Client) -> Result<Box<dyn Reply>, Infallible> {
    tracing::debug!("Pinging Database");
    warp_handle!(db::ping(&client).await);
    Ok(Box::new(StatusCode::OK))
}

pub mod people {

    use super::*;

    pub async fn create(
        client: db::Client,
        person: data::PersonRequest,
    ) -> Result<Box<dyn Reply>, Infallible> {
        tracing::info!(person_request = ?person, "Creating new Person in database");
        let reply = warp_handle!(db::create_person(&client, person).await);
        tracing::info!(person.id = ?reply, "Person Successfully Created");
        Ok(Box::new(warp::reply::json(&reply)))
    }

    pub async fn read_all(client: db::Client) -> Result<Box<dyn Reply>, Infallible> {
        tracing::info!("Querying all people from database");
        let reply = warp_handle!(db::get_people(&client).await);
        tracing::info!("Query Successful");
        Ok(Box::new(warp::reply::json(&reply)))
    }

    pub async fn read_single<T>(
        client: db::Client,
        user_id: T,
    ) -> Result<Box<dyn Reply>, Infallible>
    where
        T: AsRef<str>,
    {
        let user_id = user_id.as_ref();
        tracing::info!(person.id = ?user_id, "Querying single person from database.");
        let person = warp_handle!(db::get_person(&client, user_id).await);
        tracing::info!(peron = ?person, "Query Successful.");
        Ok(Box::new(warp::reply::json(&person)))
    }

    pub async fn update(
        client: db::Client,
        user_id: impl AsRef<str>,
        person_request: data::PersonRequest,
    ) -> Result<Box<dyn Reply>, Infallible> {
        let user_id = user_id.as_ref();
        tracing::info!(person = ?person_request, person.id = ?user_id, "Updating single person in database");
        warp_handle!(db::update_person(&client, user_id, person_request).await);
        tracing::info!("update Successful");
        Ok(Box::new(StatusCode::OK)) //return a success if the update occured
    }

    pub async fn delete(
        client: db::Client,
        user_id: impl AsRef<str>,
    ) -> Result<Box<dyn Reply>, Infallible> {
        let user_id = user_id.as_ref();
        tracing::info!(person.id = user_id, "Deleting Single person from Database");
        warp_handle!(db::delete_person(&client, user_id).await);
        tracing::info!("Deletion Successful");
        Ok(Box::new(StatusCode::OK))
    }
}
