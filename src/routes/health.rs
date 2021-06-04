use super::*;

pub fn health_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create the Health Check route
    warp::path!("health")
        .and(with_db(client))
        .and_then(handler::health)
}
