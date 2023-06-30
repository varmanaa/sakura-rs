use std::{collections::HashMap, time::Duration};

use time::OffsetDateTime;
use tokio::time::sleep;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{
    types::{
        context::Context,
        database::InviteCheckCreatePayload,
        interaction::{
            ApplicationCommandInteraction,
            DeferInteractionPayload,
            UpdateResponsePayload,
        },
        Result,
    },
    utility::{decimal::add_commas, error::Error, time::humanize},
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Run an invite check", name = "check")]
pub struct CheckCommand {}

impl CheckCommand {
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

        let (Some(cached_guild), Some(database_guild)) = (
            context.cache.get_guild(interaction.guild_id),
            context.database.get_guild(interaction.guild_id).await,
        ) else {
            return Err(Error::Custom("Please kick and invite Sakura.".to_owned()))
        };

        if cached_guild.in_check {
            return Err(Error::Custom(
                "Sakura is either running an invite check or adding a category at the
            moment. Please wait until this is done before trying again."
                    .to_owned(),
            ));
        }

        if database_guild.category_channel_ids.is_empty() {
            return Err(Error::Custom(
                "There are no categories for Sakura to check.".to_owned(),
            ));
        }

        let Some(results_channel_id) = database_guild.results_channel_id else {
            return Err(Error::Custom(
                "You have not set a \"results channel\".".to_owned(),
            ))
        };

        if context.cache.get_channel(results_channel_id).is_none() {
            return Err(Error::Custom(
                "Please set another \"results channel\".".to_owned(),
            ));
        }
        if !context
            .cache
            .has_minimum_channel_permissions(results_channel_id)
        {
            return Err(Error::Custom(format!("Sakura is unable to either view <#{results_channel_id}> or send messages in the channel.")));
        }

        context
            .cache
            .update_guild(interaction.guild_id, Some(true), None, None);

        let start_time = OffsetDateTime::now_utc();
        let start_embed = EmbedBuilder::new()
            .color(database_guild.embed_color as u32)
            .description("Sakura is checking your invites now!".to_owned())
            .build();

        if interaction.channel_id.eq(&results_channel_id) {
            interaction
                .context
                .update_response(UpdateResponsePayload {
                    embeds: vec![start_embed],
                    ..Default::default()
                })
                .await?;
        } else {
            interaction
                .context
                .update_response(UpdateResponsePayload {
                    embeds: vec![EmbedBuilder::new()
                        .color(database_guild.embed_color as u32)
                        .description(format!("Results will be sent in <#{results_channel_id}>!"))
                        .build()],
                    ..Default::default()
                })
                .await?;

            context
                .http
                .create_message(results_channel_id)
                .embeds(&[start_embed])?
                .await?;
        }

        let mut sorted_category_channels = database_guild
            .category_channel_ids
            .iter()
            .filter_map(|channel_id| {
                context.cache.get_channel(*channel_id).and_then(|channel| {
                    Some((channel.channel_id, channel.name.clone(), channel.position))
                })
            })
            .collect::<Vec<(Id<ChannelMarker>, String, i32)>>();

        sorted_category_channels.sort_unstable_by(|a, b| a.2.cmp(&b.2));

        let mut child_channels_in_categories: HashMap<
            Id<ChannelMarker>,
            Vec<(Id<ChannelMarker>, i32)>,
        > = HashMap::new();

        for channel_id in cached_guild.channel_ids.read().clone().into_iter() {
            let Some(channel) = context.cache.get_channel(channel_id) else {
                continue
            };
            let Some(parent_id) = channel.parent_id else {
                continue;
            };

            if !database_guild.category_channel_ids.contains(&parent_id) {
                continue;
            }
            if child_channels_in_categories.contains_key(&parent_id) {
                child_channels_in_categories
                    .insert(parent_id, vec![(channel_id, channel.position)]);
            }

            let Some(child_channels_in_category) = child_channels_in_categories.get_mut(&parent_id) else {
                continue;
            };

            child_channels_in_category.push((channel_id, channel.position));
        }

        let guild_invite_counts = context
            .database
            .get_guild_invite_counts(interaction.guild_id)
            .await?;
        let mut total_channels = 0u16;
        let mut total_valid = 0u16;
        let mut total_invalid = 0u16;
        let mut total_unknown = 0u16;

        for (category_channel_id, category_name, _) in sorted_category_channels {
            let description = match child_channels_in_categories.get_mut(&category_channel_id) {
                None => "No channels to check in this category.".to_owned(),
                Some(child_channels) => {
                    total_channels += child_channels.len() as u16;
                    child_channels.sort_unstable_by(|a, b| a.1.cmp(&b.1));

                    child_channels
                        .into_iter()
                        .map(|(child_channel_id, _)| {
                            if database_guild.ignored_channel_ids.contains(child_channel_id) {
                                format!("⚪ <#{child_channel_id}> - **IGNORED**")
                            } else if let Some((valid, invalid, unknown)) = guild_invite_counts.get(&child_channel_id).cloned() {
                                let total = valid + invalid + unknown;

                                total_valid += valid;
                                total_invalid += invalid;
                                total_unknown += unknown;

                                if unknown > 0 {
                                    format!("⚪ <#{child_channel_id}> - **{total}** total (**{unknown}** unknown)")
                                } else if invalid > 0 {
                                    format!(
                                        "🔴 <#{child_channel_id}> - **{total}** total (**{invalid}** invalid)"
                                    )
                                } else {
                                    format!("🟢 <#{child_channel_id}> - **{total}** total")
                                }
                            } else {
                                format!("⚪ <#{child_channel_id}> - **UNTRACKED CHANNEL**")
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                }
            };
            let embed = EmbedBuilder::new()
                .color(database_guild.embed_color as u32)
                .description(description)
                .title(format!("The \"{category_name}\" category"))
                .build();

            context
                .http
                .create_message(results_channel_id)
                .embeds(&[embed])?
                .await?;

            sleep(Duration::from_secs(1)).await;
        }

        let end_time = OffsetDateTime::now_utc();
        let end_embed = EmbedBuilder::new()
            .color(database_guild.embed_color as u32)
            .field(EmbedFieldBuilder::new(
                "Elapsed time",
                humanize(
                    ((end_time.unix_timestamp_nanos() - start_time.unix_timestamp_nanos())
                        / 1_000_000) as u128,
                ),
            ))
            .field(EmbedFieldBuilder::new(
                "Stats",
                vec![
                    format!(
                        "- **{}** channel(s) checked",
                        add_commas(total_channels as u128)
                    ),
                    format!(
                        "- **{}** invite(s) checked",
                        add_commas((total_valid + total_invalid + total_unknown) as u128)
                    ),
                    format!(
                        "- **{total_valid}** ({:.2}%) valid invite(s)",
                        (total_valid * 100) as f32
                            / (total_valid + total_invalid + total_unknown) as f32
                    ),
                    format!(
                        "- **{total_invalid}** ({:.2}%) invalid invite(s)",
                        (total_invalid * 100) as f32
                            / (total_valid + total_invalid + total_unknown) as f32
                    ),
                    format!(
                        "- **{total_unknown}** ({:.2}%) unknown invite(s)",
                        (total_unknown * 100) as f32
                            / (total_valid + total_invalid + total_unknown) as f32
                    ),
                ]
                .join("\n"),
            ))
            .title("Results")
            .build();

        context
            .http
            .create_message(results_channel_id)
            .embeds(&[end_embed])?
            .await?;
        context
            .cache
            .update_guild(interaction.guild_id, Some(false), None, None);
        context
            .database
            .insert_invite_check_create_event(InviteCheckCreatePayload {
                guild_id: interaction.guild_id.get() as i64,
                start_time,
                end_time,
                channels: total_channels as i64,
                valid_invites: total_valid as i64,
                invalid_invites: total_invalid as i64,
            })
            .await?;

        Ok(())
    }
}
