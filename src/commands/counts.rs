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
        interaction::{ApplicationCommandInteraction, UpdateResponsePayload},
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
        match context.database.get_guild(interaction.guild_id).await {
            None => {
                return Err(Error::Custom(
                    "Please kick and invite Sakura-RS.".to_owned(),
                ))
            }
            Some(database_guild) => {
                if database_guild.category_channel_ids.is_empty() {
                    return Err(Error::Custom(
                        "There are no categories for Sakura-RS to check.".to_owned(),
                    ));
                }

                let mut unsorted_category_counts: HashMap<Id<ChannelMarker>, (u8, u8, u8)> =
                    HashMap::new();

                if let Some(cached_guild) = context.cache.get_guild(interaction.guild_id) {
                    for channel_id in cached_guild.channel_ids.read().clone().into_iter() {
                        let channel = match context.cache.get_channel(channel_id) {
                            Some(channel) => channel,
                            None => continue,
                        };
                        let parent_id = match channel.parent_id {
                            Some(parent_id)
                                if database_guild.category_channel_ids.contains(&parent_id) =>
                            {
                                parent_id
                            }
                            _ => continue,
                        };

                        match unsorted_category_counts.get_mut(&parent_id) {
                            None => {
                                if channel.kind == ChannelType::GuildAnnouncement {
                                    unsorted_category_counts.insert(parent_id, (1, 0, 0));
                                }

                                if channel.kind == ChannelType::GuildText {
                                    unsorted_category_counts.insert(parent_id, (0, 1, 0));
                                }

                                if database_guild.ignored_channel_ids.contains(&channel_id) {
                                    unsorted_category_counts.insert(parent_id, (0, 0, 1));
                                }
                            }
                            Some(category_counts) => {
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
                    }
                }

                let mut sorted_category_channels = database_guild
                    .category_channel_ids
                    .iter()
                    .filter_map(|channel_id| {
                        match context.cache.get_channel(*channel_id) {
                            None => None,
                            Some(channel) => {
                                Some((channel.channel_id, channel.name.clone(), channel.position))
                            }
                        }
                    })
                    .collect::<Vec<(Id<ChannelMarker>, String, i32)>>();
                let mut embed_builder = EmbedBuilder::new().color(0xF8F8FF);

                sorted_category_channels.sort_unstable_by(|a, b| a.2.cmp(&b.2));

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
                    .update_response(UpdateResponsePayload {
                        embeds: Some(&[embed_builder.build()]),
                    })
                    .await?;
            }
        };

        Ok(())
    }
}
