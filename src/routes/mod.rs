use crate::{db, handler};
use std::convert::Infallible;
use warp::Filter;

mod health;
mod people;

pub fn routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health::health_routes(client.clone()).or(people::people_routes(client))
}

pub fn with_db(
    client: db::Client,
) -> impl Filter<Extract = (db::Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}
