use twilight_gateway::Latency;
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
    pub channel_id: Id<ChannelMarker>,
    pub data: Box<CommandData>,
    pub guild_id: Id<GuildMarker>,
    pub id: Id<InteractionMarker>,
    pub interaction_client: InteractionClient<'a>,
    pub latency: Latency,
    pub token: String,
}

pub struct DeferInteractionPayload {
    pub ephemeral: bool,
}

pub struct ResponsePayload {
    pub components: Option<Vec<Component>>,
    pub embeds: Option<Vec<Embed>>,
    pub ephemeral: bool,
}

pub struct UpdateResponsePayload<'a> {
    pub embeds: Option<&'a [Embed]>,
}
