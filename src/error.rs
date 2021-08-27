use mongodb::bson;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error Creating Database client from URI {source}")]
    ClientInitializationError { source: mongodb::error::Error },

    #[error("Error Configuring Server: {source}")]
    ConfigurationError { source: config::ConfigError },

    #[error("Could not Parse Server Configuration from \"{0}\"")]
    ServerConfigError(String),

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

    #[error("Session Error: {0}")]
    SessionError(String)
}
