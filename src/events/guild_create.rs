use std::{collections::HashSet, sync::Arc};

use time::OffsetDateTime;
use twilight_model::{
    gateway::payload::incoming::GuildCreate,
    id::{marker::UserMarker, Id},
};

use crate::types::{context::Context, database::GuildCreatePayload, Result};

pub async fn handle_guild_create(
    context: Arc<Context>,
    payload: GuildCreate,
) -> Result<()> {
    let guild_id = payload.id;
    let invite_check_category_ids =
        if let Some(database_guild) = context.database.get_guild(guild_id).await {
            database_guild.category_channel_ids
        } else {
            context
                .database
                .insert_guild_create_event(GuildCreatePayload {
                    guild_id: guild_id.get() as i64,
                })
                .await?;
            HashSet::new()
        };
    let current_user_id: Id<UserMarker> = context.application_id.cast();
    let (communication_disabled_until, role_ids) = payload
        .0
        .members
        .into_iter()
        .find(|member| member.user.id.eq(&current_user_id))
        .map_or((None, HashSet::new()), |member| {
            (
                member
                    .communication_disabled_until
                    .map_or(None, |timestamp| {
                        Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs()).unwrap())
                    }),
                HashSet::from_iter(member.roles),
            )
        });

    context.cache.insert_guild(
        payload.0.channels,
        guild_id,
        false,
        invite_check_category_ids,
        payload.0.name,
        payload.0.roles,
    );
    context.cache.insert_current_user(
        guild_id,
        communication_disabled_until,
        context.application_id.cast(),
        role_ids,
    );
    context.database.insert_guild(guild_id).await?;

    Ok(())
}
