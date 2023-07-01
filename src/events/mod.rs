mod channel_create;
mod channel_delete;
mod channel_update;
mod guild_create;
mod guild_delete;
mod guild_update;
mod interaction_create;
mod member_update;
mod message_create;
mod message_delete;
mod message_delete_bulk;
mod message_update;
mod ready;
mod role_create;
mod role_delete;
mod role_update;
mod unavailable_guild;

use std::sync::Arc;

use twilight_gateway::Event;

use self::{
    channel_create::handle_channel_create,
    channel_delete::handle_channel_delete,
    channel_update::handle_channel_update,
    guild_create::handle_guild_create,
    guild_delete::handle_guild_delete,
    guild_update::handle_guild_update,
    interaction_create::handle_interaction_create,
    member_update::handle_member_update,
    message_create::handle_message_create,
    message_delete::handle_message_delete,
    message_delete_bulk::handle_message_delete_bulk,
    message_update::handle_message_update,
    ready::handle_ready,
    role_create::handle_role_create,
    role_delete::handle_role_delete,
    role_update::handle_role_update,
    unavailable_guild::handle_unavailable_guild,
};
use crate::types::{context::Context, Result};

pub async fn handle_event(
    context: Arc<Context>,
    shard_id: u64,
    event: Event,
) -> Result<()> {
    match event {
        Event::ChannelCreate(payload) => handle_channel_create(context, *payload),
        Event::ChannelDelete(payload) => handle_channel_delete(context, *payload).await,
        Event::ChannelUpdate(payload) => handle_channel_update(context, *payload),
        Event::GuildCreate(payload) => handle_guild_create(context, *payload).await,
        Event::GuildDelete(payload) => handle_guild_delete(context, payload).await,
        Event::GuildUpdate(payload) => handle_guild_update(context, *payload),
        Event::InteractionCreate(payload) => {
            handle_interaction_create(context, shard_id, *payload).await
        }
        Event::MemberUpdate(payload) => handle_member_update(context, *payload),
        Event::MessageCreate(payload) => handle_message_create(context, *payload).await,
        Event::MessageDelete(payload) => handle_message_delete(context, payload).await,
        Event::MessageDeleteBulk(payload) => handle_message_delete_bulk(context, payload).await,
        Event::MessageUpdate(payload) => handle_message_update(context, *payload).await,
        Event::Ready(payload) => handle_ready(context, *payload),
        Event::RoleCreate(payload) => handle_role_create(context, payload),
        Event::RoleDelete(payload) => handle_role_delete(context, payload),
        Event::RoleUpdate(payload) => handle_role_update(context, payload),
        Event::UnavailableGuild(payload) => handle_unavailable_guild(context, payload),
        _ => Ok(()),
    }
}
