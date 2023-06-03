use std::collections::{HashMap, HashSet};

use time::OffsetDateTime;
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
        interaction: ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let pre_check_result: Result<(
            HashSet<Id<ChannelMarker>>,
            HashSet<Id<ChannelMarker>>,
            u32,
            HashSet<Id<ChannelMarker>>,
            Id<ChannelMarker>,
        )> = match (
            context.cache.get_guild(interaction.guild_id),
            context.database.get_guild(interaction.guild_id).await,
        ) {
            (Some(cached_guild), Some(database_guild)) => {
                if cached_guild.in_check {
                    return Err(Error::Custom(
                        "Sakura-RS is either running an invite check or adding a category at the
                    moment. Please wait until this is done before trying again."
                            .to_owned(),
                    ));
                }
                if database_guild.category_channel_ids.is_empty() {
                    return Err(Error::Custom(
                        "There are no categories for Sakura-RS to check.".to_owned(),
                    ));
                }

                let results_channel_id = match database_guild.results_channel_id {
                    Some(results_channel_id) => {
                        if context.cache.get_channel(results_channel_id).is_none() {
                            return Err(Error::Custom(
                                "Please set another \"results channel\".".to_owned(),
                            ));
                        }
                        if !context
                            .cache
                            .has_minimum_channel_permissions(results_channel_id)
                        {
                            return Err(Error::Custom(format!("Sakura-RS is unable to either view <#{results_channel_id}> or send messages in the channel.")));
                        }

                        results_channel_id
                    }
                    None => {
                        return Err(Error::Custom(
                            "You have not set a \"results channel\".".to_owned(),
                        ))
                    }
                };

                Ok((
                    cached_guild.channel_ids.read().clone(),
                    database_guild.category_channel_ids,
                    database_guild.embed_color as u32,
                    database_guild.ignored_channel_ids,
                    results_channel_id,
                ))
            }
            _ => {
                return Err(Error::Custom(
                    "Please kick and invite Sakura-RS.".to_owned(),
                ))
            }
        };
        let (
            guild_channel_ids,
            category_channel_ids,
            embed_color,
            ignored_channel_ids,
            results_channel_id,
        ) = match pre_check_result {
            Ok((
                guild_channel_ids,
                category_channel_ids,
                embed_color,
                ignored_channel_ids,
                results_channel_id,
            )) => {
                (
                    guild_channel_ids,
                    category_channel_ids,
                    embed_color,
                    ignored_channel_ids,
                    results_channel_id,
                )
            }
            Err(error) => {
                let embed = EmbedBuilder::new()
                    .color(0xF8F8FF)
                    .description(error.to_string())
                    .build();

                interaction
                    .update_response(UpdateResponsePayload {
                        embeds: Some(&[embed]),
                    })
                    .await?;

                return Ok(());
            }
        };

        context
            .cache
            .update_guild(interaction.guild_id, Some(true), None, None);

        let start_time = OffsetDateTime::now_utc();
        let start_description = if interaction.channel_id.eq(&results_channel_id) {
            "Sakura-RS is checking your invites now!".to_owned()
        } else {
            format!("Results will be sent in <#{results_channel_id}>!")
        };
        let start_embed = EmbedBuilder::new()
            .color(embed_color)
            .description(start_description)
            .build();

        interaction
            .update_response(UpdateResponsePayload {
                embeds: Some(&[start_embed]),
            })
            .await?;

        let mut invite_check_channels: HashMap<Id<ChannelMarker>, Vec<(Id<ChannelMarker>, i32)>> =
            HashMap::new();
        let mut category_channels = category_channel_ids
            .iter()
            .filter_map(|channel_id| {
                match context.cache.get_channel(*channel_id) {
                    Some(channel) => {
                        invite_check_channels.insert(channel.channel_id, Vec::new());

                        Some((channel.channel_id, channel.position))
                    }
                    None => None,
                }
            })
            .collect::<Vec<(Id<ChannelMarker>, i32)>>();

        category_channels.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        for channel_id in guild_channel_ids {
            if let Some(channel) = context.cache.get_channel(channel_id) {
                if let Some(parent_id) = channel.parent_id {
                    if category_channel_ids.contains(&parent_id) {
                        if let Some(child_channels) = invite_check_channels.get_mut(&parent_id) {
                            child_channels.push((channel_id, channel.position))
                        }
                    }
                }
            }
        }

        let guild_invite_counts = context
            .database
            .get_guild_invite_counts(interaction.guild_id)
            .await?;
        let mut total_channels = 0u16;
        let mut total_bad = 0u16;
        let mut total_good = 0u16;

        for (category_channel_id, _) in category_channels {
            let description =
                if let Some(child_channels) = invite_check_channels.get_mut(&category_channel_id) {
                    total_channels += child_channels.len() as u16;
                    child_channels.sort_unstable_by(|a, b| a.1.cmp(&b.1));

                    child_channels
                        .iter()
                        .map(|(child_channel_id, _)| {
                            if ignored_channel_ids.contains(child_channel_id) {
                                format!("⚪ <#{child_channel_id}> - **IGNORED**")
                            } else {
                                let (good, bad) = guild_invite_counts
                                    .get(&child_channel_id)
                                    .cloned()
                                    .unwrap_or((0, 0));
                                let total = good + bad;

                                total_bad += bad;
                                total_good += good;

                                if bad > 0 {
                                    format!(
                                    "🔴 <#{child_channel_id}> - **{total}** total (**{bad}** bad)"
                                )
                                } else {
                                    format!("🟢 <#{child_channel_id}> - **{total}** total")
                                }
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                } else {
                    "No channels to check in this category.".to_owned()
                };
            let embed = EmbedBuilder::new()
                .color(embed_color)
                .description(description)
                .build();

            context
                .http
                .create_message(results_channel_id)
                .embeds(&[embed])?
                .await?;
        }

        let end_time = OffsetDateTime::now_utc();
        let end_embed = EmbedBuilder::new()
            .color(embed_color)
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
                        add_commas(total_channels as f64)
                    ),
                    format!(
                        "- **{}** invite(s) checked",
                        add_commas((total_bad + total_good) as f64)
                    ),
                    format!(
                        "- **{total_bad}** ({:.2}%) invalid invite(s)",
                        (total_bad * 100) as f32 / (total_bad + total_good) as f32
                    ),
                    format!(
                        "- **{total_good}** ({:.2}%) valid invite(s)",
                        (total_good * 100) as f32 / (total_bad + total_good) as f32
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
                good_invites: total_good as i64,
                bad_invites: total_bad as i64,
            })
            .await?;

        Ok(())
    }
}