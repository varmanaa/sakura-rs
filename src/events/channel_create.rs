use std::sync::Arc;

use twilight_model::gateway::payload::incoming::ChannelCreate;

use crate::types::{context::Context, Result};

pub fn handle_channel_create(
    context: Arc<Context>,
    payload: ChannelCreate,
) -> Result<()> {
    context.cache.insert_channel(payload.0);

    Ok(())
}
