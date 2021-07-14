use mongodb::bson;
use warp::reject::Reject;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database Error: {source}")]
    DataBaseError { source: mongodb::error::Error },

    #[error("Error Configuring Server: {source}")]
    ConfigurationError { source: config::ConfigError },

    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),

    #[error("Error Constructing ObjectID from Request {0}")]
    MongoOidError(#[from] bson::oid::Error),

    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),

    #[error("Database insertion did not return a new ObjectID")]
    MongoCreateError,

    #[error("Item does not exist in collection")]
    NonexistentResourceError,
}

impl Reject for Error {}

pub type Result<T> = std::result::Result<T, Error>;
