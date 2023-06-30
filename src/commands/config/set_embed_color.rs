use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    types::{
        context::Context,
        interaction::{
            ApplicationCommandInteraction,
            DeferInteractionPayload,
            UpdateResponsePayload,
        },
        Result,
    },
    utility::error::Error,
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
        interaction: &mut ApplicationCommandInteraction<'_>,
        options: Self,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let Some(database_guild) = context.database.get_guild(interaction.guild_id).await else {
            return Err(Error::Custom(
                "Please kick and re-invite Sakura.".to_owned(),
            ))
        };
        let hex_code = format!("{:0>6}", options.hex_code.to_uppercase());

        if hex_code.chars().any(|char| char.to_digit(16).is_none()) {
            return Err(Error::Custom(format!(
                "**#{hex_code}** is not a valid hex code."
            )));
        }

        let current_hex_code = format!("{:#06}", database_guild.embed_color);

        if hex_code.eq(&current_hex_code) {
            return Err(Error::Custom(format!(
                "**#{hex_code}** is already your chosen embed color."
            )));
        }

        let color = i32::from_str_radix(&hex_code, 16)?;

        context
            .database
            .insert_embed_color(interaction.guild_id, color)
            .await?;

        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(format!(
                "The embed color for invite check embeds is now **#{hex_code}**."
            ))
            .build();

        interaction
            .context
            .update_response(UpdateResponsePayload {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
