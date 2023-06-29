use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    types::{
        context::Context,
        interaction::{ApplicationCommandInteraction, UpdateResponsePayload},
        Result,
    },
    utility::error::Error,
};

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Remove a channel from the list of ignored channels",
    name = "remove-ignored-channel"
)]
pub struct ConfigRemoveIgnoredChannelCommand {
    #[command(
        channel_types = "guild_news guild_text",
        desc = "The announcement or text channel"
    )]
    channel: Id<ChannelMarker>,
}

impl ConfigRemoveIgnoredChannelCommand {
    pub async fn run(
        context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
        options: Self,
    ) -> Result<()> {
        match context.database.get_guild(interaction.guild_id).await {
            None => {
                return Err(Error::Custom(
                    "Please kick and re-invite Sakura.".to_owned(),
                ))
            }
            Some(database_guild) => {
                let channel_id = options.channel;

                if !database_guild.ignored_channel_ids.contains(&channel_id) {
                    return Err(Error::Custom(format!(
                        "<#{channel_id}> is not an ignored channel."
                    )));
                }

                context
                    .database
                    .remove_ignored_channel(interaction.guild_id, channel_id)
                    .await?;

                let embed = EmbedBuilder::new()
                    .color(0xF8F8FF)
                    .description(format!(
                        "<#{channel_id}> will no longer be ignored during invite checks."
                    ))
                    .build();

                interaction
                    .update_response(UpdateResponsePayload {
                        embeds: Some(&[embed]),
                    })
                    .await?;
            }
        };

        Ok(())
    }
}
