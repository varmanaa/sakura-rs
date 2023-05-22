use std::sync::Arc;

use twilight_gateway::Event;

use crate::types::{context::Context, Result};

pub async fn handle_event(
    event: Event,
    context: Arc<Context>,
) -> Result<()> {
    match event {
        Event::ChannelCreate(_) => todo!(),
        Event::ChannelDelete(_) => todo!(),
        Event::ChannelUpdate(_) => todo!(),
        Event::GuildCreate(_) => todo!(),
        Event::GuildDelete(_) => todo!(),
        Event::GuildUpdate(_) => todo!(),
        Event::InteractionCreate(_) => todo!(),
        Event::MemberUpdate(_) => todo!(),
        Event::MessageCreate(_) => todo!(),
        Event::MessageDelete(_) => todo!(),
        Event::MessageDeleteBulk(_) => todo!(),
        Event::MessageUpdate(_) => todo!(),
        Event::Ready(_) => todo!(),
        Event::RoleCreate(_) => todo!(),
        Event::RoleDelete(_) => todo!(),
        Event::RoleUpdate(_) => todo!(),
        Event::UnavailableGuild(_) => todo!(),
        _ => Ok(()),
    }
}
