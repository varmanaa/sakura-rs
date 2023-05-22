use std::sync::Arc;

use twilight_http::Client;
use twilight_model::id::{marker::ApplicationMarker, Id};

use super::{cache::Cache, database::Database};

pub struct Context {
    pub application_id: Id<ApplicationMarker>,
    pub cache: Cache,
    pub database: Database,
    pub http: Arc<Client>,
}
