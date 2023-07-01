use std::collections::HashMap;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::ChannelType,
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

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

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Count channels within (added) categories", name = "counts")]
pub struct CountsCommand {}

impl CountsCommand {
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

        let Some(database_guild) = context.database.get_guild(interaction.guild_id).await else {
            return Err(Error::Custom("Please kick and invite Sakura.".to_owned()))
        };

        if database_guild.category_channel_ids.is_empty() {
            return Err(Error::Custom(
                "There are no categories for Sakura to check.".to_owned(),
            ));
        }

        let mut unsorted_category_counts: HashMap<Id<ChannelMarker>, (u8, u8, u8)> = HashMap::new();

        if let Some(cached_guild) = context.cache.get_guild(interaction.guild_id) {
            for channel_id in cached_guild.channel_ids.read().clone().into_iter() {
                let Some(channel) = context.cache.get_channel(channel_id) else {
                    continue;
                };
                let Some(parent_id) = channel.parent_id else {
                    continue;
                };

                if database_guild.category_channel_ids.contains(&parent_id) {
                    continue;
                }

                if !unsorted_category_counts.contains_key(&parent_id) {
                    unsorted_category_counts.insert(parent_id, (0, 0, 0));
                }

                let Some(category_counts) = unsorted_category_counts.get_mut(&parent_id) else {
                    continue;
                };

                if channel.kind == ChannelType::GuildAnnouncement {
                    category_counts.0 += 1;
                }

                if channel.kind == ChannelType::GuildText {
                    category_counts.1 += 1;
                }

                if database_guild.ignored_channel_ids.contains(&channel_id) {
                    category_counts.2 += 1;
                }
            }
        }

        let mut sorted_category_channels = database_guild
            .category_channel_ids
            .iter()
            .filter_map(|channel_id| {
                context
                    .cache
                    .get_channel(*channel_id)
                    .map(|channel| (channel.channel_id, channel.name.clone(), channel.position))
            })
            .collect::<Vec<(Id<ChannelMarker>, String, i32)>>();
        let mut embed_builder = EmbedBuilder::new().color(0xF8F8FF);

        sorted_category_channels.sort_unstable_by(|a, b| {
            let sort_ordering = a.2.cmp(&b.2);

            if !sort_ordering.is_eq() {
                a.1.cmp(&b.1)
            } else {
                sort_ordering
            }
        });

        for sorted_category_channel in sorted_category_channels {
            let (announcement, text, ignored) = unsorted_category_counts
                .get(&sorted_category_channel.0)
                .cloned()
                .unwrap_or((0, 0, 0));
            let value = vec![
                format!("- {announcement} announcement channel(s)"),
                format!("- {text} text channel(s)"),
                format!("- {ignored} ignored channel(s)"),
            ]
            .join("\n");

            embed_builder = embed_builder.field(EmbedFieldBuilder::new(
                format!("The \"{}\" category", sorted_category_channel.1),
                value,
            ));
        }

        interaction
            .context
            .update_response(UpdateResponsePayload {
                embeds: vec![embed_builder.build()],
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
