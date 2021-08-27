use warp::filters::{body, path};
use warp::Filter;

use super::{with_db, with_optional_session};
use crate::{data, db, handler};

pub fn todo_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(warp::path("todos"))
        .and(with_db(client.clone()))
        .and(with_optional_session())
        .and_then(handler::todos::get_todos)
}
