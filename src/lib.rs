pub mod config;
pub mod db;
pub mod error;
pub mod routes;
pub mod startup;

pub mod data;
mod handler;

type Result<T> = std::result::Result<T, error::Error>;
