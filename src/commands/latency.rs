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
        context: &Context,
        interaction: &ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let response = interaction.context.response().await?;
        let rtt_ms = (((response.id.get() >> 22) + 1_420_070_400_000)
            - ((interaction.context.id.get() >> 22) + 1_420_070_400_000))
            .to_string();
        let rtt_description = format!("🚀 **RTT**: {} ms", add_commas(rtt_ms));
        let shard_ping_description = if let Some(latency) = context.latency(interaction.shard_id) {
            latency.average().map_or("".to_owned(), |duration| {
                let duration_ms = duration.as_millis().to_string();

                format!("🏓 **Shard:** {} ms", add_commas(duration_ms))
            })
        } else {
            "".to_owned()
        };
        let description = vec![shard_ping_description, rtt_description]
            .join("\n")
            .trim()
            .to_owned();
        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(description)
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
