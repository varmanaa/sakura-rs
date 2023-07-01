use std::sync::Arc;

use twilight_model::gateway::payload::incoming::ChannelUpdate;

use crate::types::{cache, context::Context, Result};

pub fn handle_channel_update(
    context: Arc<Context>,
    payload: ChannelUpdate,
) -> Result<()> {
    context.cache.update_channel(
        payload.id,
        cache::ChannelUpdate {
            kind: Some(payload.kind),
            name: Some(payload.name.clone().unwrap_or_default()),
            parent_id: Some(payload.parent_id),
            permission_overwrites: Some(payload.permission_overwrites.clone()),
            position: Some(payload.position.unwrap_or_default()),
        },
    );
    Ok(())
}
