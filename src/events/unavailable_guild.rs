use std::sync::Arc;

use twilight_model::gateway::payload::incoming::UnavailableGuild;

use crate::types::{context::Context, Result};

pub fn handle_unavailable_guild(
    context: Arc<Context>,
    payload: UnavailableGuild,
) -> Result<()> {
    context.cache.insert_unavailable_guild(payload.id);

    Ok(())
}
