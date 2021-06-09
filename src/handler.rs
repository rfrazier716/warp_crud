use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::error::ServerError::DataBaseError;
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

    pub async fn create(
        client: db::Client,
        person: data::PersonRequest,
    ) -> Result<impl Reply, Rejection> {
        //TODO!: Finish this Function
        Ok(StatusCode::OK)
    }

    pub async fn read<T>(client: db::Client, name_str: T) -> Result<impl Reply, Rejection>
    where
        T: AsRef<str>,
    {
        let name_str = name_str.as_ref();
        print!("{:?}", name_str);
        // build the request filter
        let request_filter = if name_str.is_empty() {
            None
        } else {
            Some(doc! {"fname": name_str})
        };

        let reply = db::get_people(&client, request_filter)
            .await
            .map_err(|source| DataBaseError { source })?;
        Ok(warp::reply::json(&reply))
    }
}
