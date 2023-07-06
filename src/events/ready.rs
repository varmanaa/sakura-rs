use std::sync::Arc;

use time::OffsetDateTime;
use twilight_model::gateway::payload::incoming::Ready;

use crate::types::{context::Context, Result};

pub fn handle_ready(
    context: Arc<Context>,
    payload: Ready,
) -> Result<()> {
    for unvailable_guild in payload.guilds.into_iter() {
        context.cache.insert_unavailable_guild(unvailable_guild.id);
    }

    *context.ready_at.write() = Some(OffsetDateTime::now_utc());

    println!(
        "{}#{:04} is ready!",
        payload.user.name, payload.user.discriminator
    );

    Ok(())
}
