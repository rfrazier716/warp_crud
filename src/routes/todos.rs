use warp::filters::body;
use warp::Filter;

use super::{with_db, with_optional_session, with_required_session};
use crate::{data, db, handler};

pub fn todo_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todo = warp::path("api")
        .and(warp::path("todos"))
        .and(with_db(client));

    todo.clone()
        .and(with_optional_session())
        .and(warp::get())
        .and_then(handler::todos::get_todos)
        .or(todo
            .clone()
            .and(with_required_session())
            .and(warp::post())
            .and(todo_request().map(|request: data::TodoRequest| request.into()))
            .and_then(handler::todos::create_todo))
        .or(todo
            .clone()
            .and(with_required_session())
            .and(warp::delete())
            .and(warp::path::param::<uuid::Uuid>())
            .and_then(handler::todos::delete_todo))
        .or(todo
            .clone()
            .and(with_required_session())
            .and(warp::path::param::<uuid::Uuid>())
            .and(warp::put())
            .and(todo_request())
            .and_then(handler::todos::update_todo))
        .or(todo
            .clone()
            .and(with_required_session())
            .and(warp::delete())
            .and_then(handler::todos::delete_all_todos))
}

fn todo_request() -> impl Filter<Extract = (data::TodoRequest,), Error = warp::Rejection> + Clone {
    body::content_length_limit(4096).and(body::json::<data::TodoRequest>())
}
