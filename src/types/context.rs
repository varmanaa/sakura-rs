use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use time::OffsetDateTime;
use twilight_gateway::Latency;
use twilight_http::Client;
use twilight_model::id::{marker::ApplicationMarker, Id};

use super::{cache::Cache, database::Database};

pub struct Context {
    pub application_id: Id<ApplicationMarker>,
    pub cache: Cache,
    pub database: Database,
    pub http: Arc<Client>,
    pub shards: RwLock<HashMap<u64, Arc<Shard>>>,
}

#[derive(Debug)]
pub struct Shard {
    pub latency: Latency,
    pub ready_at: Option<OffsetDateTime>,
    pub shard_id: u64,
}
