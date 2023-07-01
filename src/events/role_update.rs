use std::sync::Arc;

use twilight_model::gateway::payload::incoming::RoleUpdate;

use crate::types::{cache, context::Context, Result};

pub fn handle_role_update(
    context: Arc<Context>,
    payload: RoleUpdate,
) -> Result<()> {
    context.cache.update_role(
        payload.role.id,
        cache::RoleUpdate {
            permissions: Some(payload.role.permissions),
        },
    );
    Ok(())
}
