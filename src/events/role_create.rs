use std::sync::Arc;

use twilight_model::gateway::payload::incoming::RoleCreate;

use crate::types::{context::Context, Result};

pub fn handle_role_create(
    context: Arc<Context>,
    payload: RoleCreate,
) -> Result<()> {
    context.cache.insert_role(payload.guild_id, payload.role);

    Ok(())
}
