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
    desc = "Remove a channel from the list of added category channels",
    name = "remove-category-channel"
)]
pub struct ConfigRemoveCategoryChannelCommand {
    #[command(channel_types = "guild_category", desc = "The category channel")]
    channel: Id<ChannelMarker>,
}

impl ConfigRemoveCategoryChannelCommand {
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

        let cached_guild = match context.cache.get_guild(interaction.guild_id) {
            Some(cached_guild) => cached_guild,
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
        let description = if !cached_guild
            .invite_check_category_ids
            .read()
            .contains(&channel_id)
        {
            format!("<#{channel_id}> is not an added category.")
        } else {
            let updated_category_channel_ids = context
                .database
                .remove_category_channel(interaction.guild_id, channel_id)
                .await?;

            context.cache.update_guild(
                interaction.guild_id,
                None,
                Some(updated_category_channel_ids),
                None,
            );

            format!("<#{channel_id}> will no longer be checked during invite checks.")
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
