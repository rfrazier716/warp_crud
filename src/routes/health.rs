use super::*;

pub fn health_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create the Health Check route
    warp::path!("health")
        .and(with_db(client))
        .and_then(handler::health)
        .with(warp::trace(|info| {
            // Construct our own custom span for this route.
            tracing::info_span!("Health Check", req.path = ?info.path())
        }))
}
