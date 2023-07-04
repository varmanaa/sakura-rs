use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use time::OffsetDateTime;
use twilight_gateway::Latency;
use twilight_http::client::{Client, InteractionClient};
use twilight_model::id::{marker::ApplicationMarker, Id};

use crate::types::{cache::Cache, context::Context, database::Database};

impl Context {
    pub fn interaction_client(&self) -> InteractionClient<'_> {
        self.http.interaction(self.application_id)
    }

    pub fn new(
        application_id: Id<ApplicationMarker>,
        cache: Cache,
        database: Database,
        http: Client,
    ) -> Self {
        Self {
            application_id,
            cache,
            database,
            http: Arc::new(http),
            latencies: RwLock::new(HashMap::new()),
            ready_at: RwLock::new(None),
        }
    }

    pub fn ready_at(&self) -> Option<OffsetDateTime> {
        *self.ready_at.read()
    }

    pub fn latency(
        &self,
        shard_id: u64,
    ) -> Option<Arc<Latency>> {
        self.latencies.read().get(&shard_id).cloned()
    }
}
