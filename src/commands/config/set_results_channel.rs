use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::types::{
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdateResponsePayload},
    Result,
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
        interaction: ApplicationCommandInteraction<'_>,
        options: Self,
    ) -> Result<()> {
        interaction
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let database_guild = match context.database.get_guild(interaction.guild_id).await {
            Some(database_guild) => database_guild,
            None => {
                let embed = EmbedBuilder::new()
                    .color(0xF8F8FF)
                    .description("Please kick and invite Sakura-RS.")
                    .build();

                interaction
                    .update_response(UpdateResponsePayload {
                        embeds: Some(&[embed]),
                    })
                    .await?;

                return Ok(());
            }
        };
        let channel_id = options.channel;
        let description = match database_guild.results_channel_id {
            Some(results_channel_id) if results_channel_id.eq(&channel_id) => {
                format!("<#{channel_id}> is already set as your results channel.")
            }
            _ => {
                context
                    .database
                    .insert_results_channel(interaction.guild_id, channel_id)
                    .await?;

                format!("Invite check results will now be sent in <#{channel_id}>.")
            }
        };
        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(description)
            .build();

        interaction
            .update_response(UpdateResponsePayload {
                embeds: Some(&[embed]),
            })
            .await?;

        Ok(())
    }
}
