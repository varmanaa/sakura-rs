use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    types::{
        context::Context,
        interaction::{ApplicationCommandInteraction, UpdateResponsePayload},
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
        interaction: &ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        let response = interaction.response().await?;
        let rtt_ms = (((response.id.get() >> 22) + 1_420_070_400_000)
            - ((interaction.id.get() >> 22) + 1_420_070_400_000)) as u128;
        let rtt_ms_with_commas = add_commas(rtt_ms);
        let description = interaction.latency.average().map_or(
            format!("🚀 **RTT**: {rtt_ms_with_commas} ms"),
            |duration| {
                let duration_ms = duration.as_millis();
                let duration_ms_with_commas = add_commas(duration_ms);

                format!(
                    "🏓 **Shard:** {} ms\n🚀 **RTT**: {rtt_ms_with_commas} ms",
                    duration_ms_with_commas
                )
            },
        );
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
