use std::{collections::HashSet, sync::Arc};

use time::OffsetDateTime;
use twilight_model::{
    gateway::payload::incoming::MemberUpdate,
    id::{marker::UserMarker, Id},
};

use crate::types::{cache::CurrentUserUpdate, context::Context, Result};

pub fn handle_member_update(
    context: Arc<Context>,
    payload: MemberUpdate,
) -> Result<()> {
    let current_user_id: Id<UserMarker> = context.application_id.cast();

    if payload.user.id.eq(&current_user_id) {
        let guild_id = payload.guild_id;
        let communication_disabled_until = payload
            .communication_disabled_until
            .map_or(None, |timestamp| {
                Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs()).unwrap())
            });
        let role_ids = HashSet::from_iter(payload.roles);

        context.cache.update_current_user(
            guild_id,
            CurrentUserUpdate {
                communication_disabled_until: Some(communication_disabled_until),
                role_ids: Some(role_ids),
            },
        )
    }

    Ok(())
}
