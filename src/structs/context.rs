use std::sync::Arc;

use twilight_http::{client::InteractionClient, Client};
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
        http: Arc<Client>,
    ) -> Self {
        Self {
            application_id,
            cache,
            database,
            http,
        }
    }
}
