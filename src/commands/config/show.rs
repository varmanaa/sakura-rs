use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::types::{
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdateResponsePayload},
    Result,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Show your server's current configuration", name = "show")]
pub struct ConfigShowCommand {}

impl ConfigShowCommand {
    pub async fn run(
        context: &Context,
        interaction: ApplicationCommandInteraction<'_>,
        _options: Self,
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
        let category_channel_ids_text = if guild.category_channel_ids.is_empty() {
            "No categories added.".to_string()
        } else {
            guild
                .category_channel_ids
                .iter()
                .map(|channel_id| {
                    context
                        .cache
                        .get_channel(*channel_id)
                        .map_or(format!("{channel_id} **(no longer exists)**"), |_| {
                            format!("<#{channel_id}>")
                        })
                })
                .collect::<Vec<String>>()
                .join("\n")
        };
        let color_text = format!("#{:06X}", guild.embed_color);
        let ignored_channel_ids_text = if guild.ignored_channel_ids.is_empty() {
            "No channels ignored.".to_string()
        } else {
            guild
                .ignored_channel_ids
                .iter()
                .map(|channel_id| {
                    context
                        .cache
                        .get_channel(*channel_id)
                        .map_or(format!("{channel_id} **(no longer exists)**"), |_| {
                            format!("<#{channel_id}>")
                        })
                })
                .collect::<Vec<String>>()
                .join("\n")
        };
        let result_text = guild
            .results_channel_id
            .map_or("No results channel set.".to_string(), |channel_id| {
                format!("<#{}>", channel_id)
            });
        let embed = EmbedBuilder::new()
            .color(guild.embed_color as u32)
            .field(EmbedFieldBuilder::new("Categories", category_channel_ids_text).build())
            .field(EmbedFieldBuilder::new("Embed color", color_text).build())
            .field(EmbedFieldBuilder::new("Ignored", ignored_channel_ids_text).build())
            .field(EmbedFieldBuilder::new("Results channel", result_text).build())
            .build();

        interaction
            .update_response(UpdateResponsePayload {
                embeds: Some(&[embed]),
            })
            .await?;

        Ok(())
    }
}
