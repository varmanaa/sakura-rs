use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use twilight_http::{client::InteractionClient, Client};
use twilight_model::id::{marker::ApplicationMarker, Id};

use crate::types::context::Shard;
use crate::types::{cache::Cache, context::Context, database::Database};

impl Context {
    pub fn interaction_client(&self) -> InteractionClient<'_> {
        self.http.interaction(self.application_id)
    }

    pub fn new(
        application_id: Id<ApplicationMarker>,
        cache: Cache,
        database: Database,
        http: Arc<Client>,
    ) -> Self {
        Self {
            application_id,
            cache,
            database,
            http,
            shards: RwLock::new(HashMap::new()),
        }
    }

    pub fn shard(
        &self,
        shard_id: u64,
    ) -> Option<Arc<Shard>> {
        self.shards.read().get(&shard_id).cloned()
    }
}
