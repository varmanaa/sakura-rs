use std::sync::Arc;

use twilight_model::gateway::payload::incoming::ChannelDelete;

use crate::types::{cache::GuildUpdate, context::Context, Result};

pub async fn handle_channel_delete(
    context: Arc<Context>,
    payload: ChannelDelete,
) -> Result<()> {
    let channel_id = payload.id;

    if let Some(guild_id) = payload.guild_id {
        context.cache.remove_channel(payload.id);

        if let Ok(updated_category_channel_ids) =
            context.database.remove_channel(guild_id, channel_id).await
        {
            context.cache.update_guild(
                guild_id,
                GuildUpdate {
                    invite_check_category_ids: Some(updated_category_channel_ids),
                    ..Default::default()
                },
            )
        }

        context.database.remove_channel_messages(channel_id).await?;
    }

    Ok(())
}
