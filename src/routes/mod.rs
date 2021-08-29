use crate::{data, db, handler};
use std::convert::Infallible;
use tracing::field::{display, Empty};
use warp::filters::cookie;
use warp::Filter;

mod health;
mod people;
mod todos;

pub fn routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base_route = warp::fs::dir("static");

    health::health_routes(client.clone())
        .or(todos::todo_routes(client.clone()))
        .or(people::people_routes(client))
        .or(base_route)
        .with(warp::trace(|info| {
            let span = tracing::info_span!(
                "request",
                id = rand::random::<u32>(),
                method = %info.method(),
                path = %info.path(),
                version = ?info.version(),
                remote.addr = Empty,
                referer = Empty,
            );

            // Record optional fields.
            if let Some(remote_addr) = info.remote_addr() {
                span.record("remote.addr", &display(remote_addr));
            }

            if let Some(referer) = info.referer() {
                span.record("referer", &display(referer));
            }

            tracing::debug!(parent: &span, "received request");

            span
        }))
}

pub fn with_db(
    client: db::Client,
) -> impl Filter<Extract = (db::Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn with_optional_session(
) -> impl Filter<Extract = (Option<data::Session>,), Error = Infallible> + Copy {
    cookie::optional::<uuid::Uuid>("session").map(|cookie: Option<uuid::Uuid>| {
        if let Some(id) = cookie {
            Some(id.into())
        } else {
            None
        }
    })
}

pub fn with_required_session(
) -> impl Filter<Extract = (data::Session,), Error = warp::Rejection> + Copy {
    cookie::cookie::<uuid::Uuid>("session").map(|cookie: uuid::Uuid| cookie.into())
}
