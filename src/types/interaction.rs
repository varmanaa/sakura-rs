use twilight_http::client::InteractionClient;
use twilight_model::{
    application::interaction::application_command::CommandData,
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
    pub token: String,
}
