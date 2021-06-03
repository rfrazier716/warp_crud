use super::*;

pub fn people_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let people = warp::path("people");

    people
        .and(warp::get())
        .and(with_db(client.clone()))
        .and_then(read_people)
}

pub async fn read_people(client: db::Client) -> Result<impl Reply, Rejection> {
    let reply =db::get_people(&client)
        .await
        .map_err(|source| error::ServerError::DataBaseError { source })?;
    Ok(warp::reply::json(&reply))
}
