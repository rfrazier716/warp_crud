use super::*;
use crate::error::ServerError::DataBaseError;
use mongodb::bson::doc;
use warp::filters::path;
use warp::{reject, Filter, Rejection, Reply};

pub fn people_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let people = warp::path("api").and(warp::path("people"));
    people
        .and(warp::get())
        .and(with_db(client.clone()))
        .and(
            path::param::<String>()
                .or(warp::any().map(|| String::from("")))
                .unify(),
        )
        .and(path::end())
        .and_then(get_people)
}

pub async fn get_people<T>(client: db::Client, name_str: T) -> Result<impl Reply, Rejection>
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
