use std::sync::Arc;

use twilight_model::gateway::payload::incoming::GuildUpdate;

use crate::types::{cache, context::Context, Result};

pub fn handle_guild_update(
    context: Arc<Context>,
    payload: GuildUpdate,
) -> Result<()> {
    context.cache.update_guild(
        payload.id,
        cache::GuildUpdate {
            name: Some(payload.name.clone()),
            ..Default::default()
        },
    );

    Ok(())
}
