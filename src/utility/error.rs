use thiserror::Error;

#[derive(Debug, Error)]

pub enum Error {
    #[error("Unable to build connection pool")]
    Build(#[from] deadpool_postgres::BuildError),
    #[error("Unable to retrieve object from pool")]
    Pool(#[from] deadpool_postgres::PoolError),
    #[error("PostgreSQL error")]
    PostgreSQL(#[from] tokio_postgres::Error),
    #[error("Unable to convert data to JSON(B) format")]
    SerdeJson(#[from] serde_json::Error),
}
