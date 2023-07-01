use std::sync::Arc;

use twilight_model::gateway::payload::incoming::GuildDelete;

use crate::types::{context::Context, database::GuildDeletePayload, Result};

pub async fn handle_guild_delete(
    context: Arc<Context>,
    payload: GuildDelete,
) -> Result<()> {
    let guild_id = payload.id;

    context.cache.remove_guild(guild_id, payload.unavailable);
    context.database.remove_guild(guild_id).await?;
    context.database.remove_guild_messages(guild_id).await?;
    context
        .database
        .insert_guild_delete_event(GuildDeletePayload {
            guild_id: guild_id.get() as i64,
        })
        .await?;

    Ok(())
}
