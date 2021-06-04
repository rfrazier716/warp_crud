use super::*;
use crate::error::ServerError::DataBaseError;
use mongodb::bson::doc;
use warp::{reject, Filter, Rejection, Reply};

pub fn people_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let people = warp::path("api").and(warp::path("people"));
    people
        .and(warp::get())
        .and(with_db(client.clone()))
        .and(warp::path::param::<String>())
        .and_then(get_people)
}

// pub async fn read_people(client: db::Client) -> Result<impl Reply, Rejection> {
//     tracing::span!(tracing::Level::INFO, "getting all people");
//     let reply = db::get_people(&client)
//         .await
//         .map_err(|source| error::ServerError::DataBaseError { source })?;
//     Ok(warp::reply::json(&reply))
// }

pub async fn get_people<T>(client: db::Client, name_str: T) -> Result<impl Reply, Rejection>
where
    T: AsRef<str>,
{
    // build the request filter
    let request_filter = if name_str.as_ref().is_empty() {
        None
    } else {
        Some(doc! {"fname": name_str.as_ref()})
    };

    let reply = db::get_people(&client, request_filter)
        .await
        .map_err(|source| error::ServerError::DataBaseError { source })?;
    Ok(warp::reply::json(&reply))
}
pub async fn read_person<T>(client: db::Client, name_str: T) -> Result<impl Reply, Rejection>
where
    T: AsRef<str>,
{
    tracing::span!(tracing::Level::INFO, "getting person");
    if let Some(reply) = db::get_person(&client, name_str.as_ref())
        .await
        .map_err(|source| DataBaseError { source })?
    {
        Ok(warp::reply::json(&reply))
    } else {
        Err(warp::reject::not_found())
    }
}
