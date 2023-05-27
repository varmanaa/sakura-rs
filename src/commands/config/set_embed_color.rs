use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedBuilder;

use crate::types::{
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdateResponsePayload},
    Result,
};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(
    desc = "Set the embed color to use for invite check embeds",
    name = "set-embed-color"
)]
pub struct ConfigSetEmbedColorCommand {
    #[command(
        desc = "The (hex) color code (without the leading hashtag)",
        rename = "hex-code"
    )]
    hex_code: String,
}

impl ConfigSetEmbedColorCommand {
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
        let current_hex_code = format!("{:#06}", guild.embed_color);
        let hex_code = format!("{:0>6}", options.hex_code.to_uppercase());
        let description = if hex_code.chars().any(|char| char.to_digit(16).is_none()) {
            format!("**#{hex_code}** is not a valid hex code.")
        } else if current_hex_code == hex_code {
            format!("**#{hex_code}** is already your chosen embed color.")
        } else {
            let color = i32::from_str_radix(&hex_code, 16).unwrap();

            context
                .database
                .insert_embed_color(interaction.guild_id, color)
                .await?;

            format!("The embed color for invite check embeds is now **#{hex_code}**.")
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
