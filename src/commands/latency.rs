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
    utility::decimal::add_commas,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Check Discord API latency", name = "latency")]
pub struct LatencyCommand {}

impl LatencyCommand {
    pub async fn run(
        _context: &Context,
        interaction: ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let response = interaction.response().await?;
        let rtt = add_commas(
            (((response.id.get() >> 22) + 1_420_070_400_000)
                - ((interaction.id.get() >> 22) + 1_420_070_400_000)) as f64,
        );
        let description = format!("**RTT**: {rtt}");
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
