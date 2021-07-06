use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::error::Error::*;
use crate::{data, db};

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

    pub async fn read_single<T>(client: db::Client, user_id: T) -> Result<Box<dyn Reply>, Rejection>
    where
        T: AsRef<str>,
    {
        let reply = db::get_person(&client, user_id.as_ref()).await;

        // if it's an OID conversion error we want that to show up as a generic 404
        let result = match reply {
            Err(MongoOidError(_)) => None,
            other => other?,
        };

        // Implement the result, with a success or 404 error
        if let Some(person) = result {
            Ok(Box::new(warp::reply::json(&person)))
        } else {
            Ok(Box::new(
                warp::http::Response::builder()
                    .status(404)
                    .body("404: Person not found"),
            ))
        } // if there wasn't a person with that ID return a 404 error
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
}
