use memory_stats::memory_stats;
use time::OffsetDateTime;
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
    utility::{decimal::add_commas, time::humanize},
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Display statistics about Sakura", name = "stats")]
pub struct StatsCommand {}

impl StatsCommand {
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

        let memory_description = if let Some(usage) = memory_stats() {
            let memory_string =
                add_commas(format!("{:.2}", (usage.physical_mem as f32 / 1_000_000f32)));

            format!("**Memory usage:** {memory_string} MB")
        } else {
            "".to_owned()
        };
        let uptime_description = if let Some(ready_at) = context.ready_at() {
            let uptime_string = humanize(
                ((OffsetDateTime::now_utc().unix_timestamp() - ready_at.unix_timestamp()) * 1000)
                    .try_into()
                    .unwrap(),
            );

            format!("**Uptime:** {uptime_string}")
        } else {
            "".to_owned()
        };
        let description = vec![
            format!(
                "**Guilds:** {}",
                add_commas(context.cache.guilds.read().len().to_string())
            ),
            format!(
                "**Channels:** {}",
                add_commas(context.cache.channels.read().len().to_string())
            ),
            memory_description,
            uptime_description,
        ]
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
