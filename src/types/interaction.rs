use twilight_http::client::InteractionClient;
use twilight_model::{
    application::interaction::application_command::CommandData,
    channel::message::{Component, Embed},
    id::{
        marker::{ChannelMarker, GuildMarker, InteractionMarker},
        Id,
    },
};

pub struct ApplicationCommandInteraction<'a> {
    pub context: ApplicationCommandInteractionContext<'a>,
    pub channel_id: Id<ChannelMarker>,
    pub data: Box<CommandData>,
    pub guild_id: Id<GuildMarker>,
}

pub struct ApplicationCommandInteractionContext<'a> {
    pub id: Id<InteractionMarker>,
    pub interaction_client: InteractionClient<'a>,
    pub token: String,
}

#[derive(Default)]
pub struct DeferInteractionPayload {
    pub ephemeral: bool,
}

#[derive(Default)]
pub struct ResponsePayload {
    pub components: Vec<Component>,
    pub embeds: Vec<Embed>,
    pub ephemeral: bool,
}

#[derive(Default)]
pub struct UpdateResponsePayload {
    pub components: Vec<Component>,
    pub embeds: Vec<Embed>,
}
