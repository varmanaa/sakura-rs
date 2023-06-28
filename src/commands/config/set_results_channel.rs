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
    desc = "Set the channel to send invite check results to",
    name = "set-results-channel"
)]
pub struct ConfigSetResultsChannelCommand {
    #[command(channel_types = "guild_news guild_text", desc = "The text channel")]
    channel: Id<ChannelMarker>,
}

impl ConfigSetResultsChannelCommand {
    pub async fn run(
        context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
        options: Self,
    ) -> Result<()> {
        match context.database.get_guild(interaction.guild_id).await {
            None => {
                return Err(Error::Custom(
                    "Please kick and re-invite Sakura-RS.".to_owned(),
                ))
            }
            Some(database_guild) => {
                let channel_id = options.channel;
                if database_guild
                    .results_channel_id
                    .map_or(false, |results_channel_id| {
                        channel_id.eq(&results_channel_id)
                    })
                {
                    return Err(Error::Custom(format!(
                        "<#{channel_id}> is already set as your results channel."
                    )));
                }
                context
                    .database
                    .insert_results_channel(interaction.guild_id, channel_id)
                    .await?;

                let embed = EmbedBuilder::new()
                    .color(0xF8F8FF)
                    .description(format!(
                        "Invite check results will now be sent in <#{channel_id}>."
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
