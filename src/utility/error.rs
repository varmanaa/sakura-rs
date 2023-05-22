use thiserror::Error;

#[derive(Debug, Error)]

pub enum Error {
    #[error("Unable to build connection pool")]
    Build(#[from] deadpool_postgres::BuildError),
    #[error("Unable to make deserialize response body")]
    Deserialization(#[from] twilight_http::response::DeserializeBodyError),
    #[error("Environment variable is not set")]
    EnvironmentVariable(#[from] std::env::VarError),
    #[error("Unable to make HTTP request to Discord")]
    Http(#[from] twilight_http::error::Error),
    #[error("Unable to retrieve object from pool")]
    Pool(#[from] deadpool_postgres::PoolError),
    #[error("PostgreSQL error")]
    PostgreSQL(#[from] tokio_postgres::Error),
    #[error("Unable to convert data to JSON(B) format")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Unable to fetch recommended number of shards to use")]
    StartRecommended(#[from] twilight_gateway::stream::StartRecommendedError),
}
