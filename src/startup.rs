use crate::{config, db, error, routes};
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tokio::signal;

// Run is in its own function so it can be started as a separate task for Integration Tests
pub async fn run(
    settings: config::Settings,
) -> Result<(SocketAddr, impl Future<Output = ()> + 'static), error::Error> {
    // Create a Database Connection from the URI
    let client = db::Client::with_uri_str(settings.database.uri)
        .await
        .map_err(|source| error::Error::ClientInitializationError { source })?;

    // Add all our routes
    let routes = routes::routes(client);

    // Create a Socket to bind the server to
    let socket = SocketAddr::new(
        IpAddr::V4(
            Ipv4Addr::from_str(&settings.server.address)
                .expect("Could Not Parse IP Address from Configuration"),
        ),
        settings.server.application_port,
    );

    //Start the Server
    Ok(
        warp::serve(routes).bind_with_graceful_shutdown(socket, async {
            match signal::ctrl_c().await {
                Ok(_) => println!("💀 Shutting Down Server"),
                Err(_) => println!("Error handling SIGINT"),
            }
        }),
    )
}
