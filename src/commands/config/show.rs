use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{
    types::{
        context::Context,
        interaction::{ApplicationCommandInteraction, UpdateResponsePayload},
        Result,
    },
    utility::error::Error,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Show your server's current configuration", name = "show")]
pub struct ConfigShowCommand {}

impl ConfigShowCommand {
    pub async fn run(
        context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
        _options: Self,
    ) -> Result<()> {
        match context.database.get_guild(interaction.guild_id).await {
            None => {
                return Err(Error::Custom(
                    "Please kick and re-invite Sakura-RS.".to_owned(),
                ))
            }
            Some(database_guild) => {
                let category_channel_ids_text =
                    if database_guild.category_channel_ids.is_empty() {
                        "No categories added.".to_string()
                    } else {
                        database_guild
                            .category_channel_ids
                            .iter()
                            .map(|channel_id| {
                                context.cache.get_channel(*channel_id).map_or(
                                    format!("- {channel_id} **(no longer exists)**"),
                                    |_| format!("- <#{channel_id}>"),
                                )
                            })
                            .collect::<Vec<String>>()
                            .join("\n")
                    };
                let color_text = format!("#{:06X}", database_guild.embed_color);
                let ignored_channel_ids_text =
                    if database_guild.ignored_channel_ids.is_empty() {
                        "No channels ignored.".to_string()
                    } else {
                        database_guild
                            .ignored_channel_ids
                            .iter()
                            .map(|channel_id| {
                                context.cache.get_channel(*channel_id).map_or(
                                    format!("- {channel_id} **(no longer exists)**"),
                                    |_| format!("- <#{channel_id}>"),
                                )
                            })
                            .collect::<Vec<String>>()
                            .join("\n")
                    };
                let result_text = database_guild
                    .results_channel_id
                    .map_or("No results channel set.".to_string(), |channel_id| {
                        format!("<#{}>", channel_id)
                    });

                let embed = EmbedBuilder::new()
                    .color(database_guild.embed_color as u32)
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
            }
        };

        Ok(())
    }
}
