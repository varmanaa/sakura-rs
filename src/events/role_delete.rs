use std::sync::Arc;

use twilight_model::gateway::payload::incoming::RoleDelete;

use crate::types::{context::Context, Result};

pub fn handle_role_delete(
    context: Arc<Context>,
    payload: RoleDelete,
) -> Result<()> {
    context.cache.remove_role(payload.role_id);

    Ok(())
}
