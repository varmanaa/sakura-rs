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
    desc = "Add a channel for Sakura-RS to ignore during an invite check",
    name = "add-ignored-channel"
)]
pub struct ConfigAddIgnoredChannelCommand {
    #[command(
        channel_types = "guild_news guild_text",
        desc = "The announcement or text channel"
    )]
    channel: Id<ChannelMarker>,
}

impl ConfigAddIgnoredChannelCommand {
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

        let guild = match context.database.get_guild(interaction.guild_id).await {
            Some(guild) => guild,
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
        let description = if guild.ignored_channel_ids.contains(&channel_id) {
            format!("<#{channel_id}> is already an ignored channel.")
        } else {
            context
                .database
                .insert_ignored_channel(interaction.guild_id, channel_id)
                .await?;

            format!("<#{channel_id}> will now be ignored during invite checks.")
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
