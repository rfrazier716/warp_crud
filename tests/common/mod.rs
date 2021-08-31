#[cfg(test)]
use std::net::SocketAddr;
use tokio;
use warp_crud::{config, error::Result, startup};

pub struct App {
    address: SocketAddr,
}

impl App {
    pub async fn launch(run_environment: Option<&str>) -> Result<App> {
        let env = match run_environment {
            Some(env) => env,
            None => "Test",
        };

        // Set the environment so the right config is loaded
        std::env::set_var("RUN_ENV", env);
        let app_settings = config::Settings::new()?;
        let (addr, server) = startup::run(app_settings).await?;
        tokio::task::spawn(server);
        Ok(App { address: addr })
    }

    pub fn route(self, endpoint: &str) -> String {
        format!(
            "http://{}:{}{}",
            self.address.ip(),
            self.address.port(),
            endpoint
        )
    }
}
