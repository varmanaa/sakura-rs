use std::{collections::HashSet, mem::take, sync::Arc};

use time::OffsetDateTime;
use twilight_gateway::Event;
use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    channel::ChannelType,
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    commands::{
        check::CheckCommand,
        check_message::CheckMessageCommand,
        config::ConfigCommand,
        counts::CountsCommand,
        info::InfoCommand,
    },
    types::{
        context::Context,
        database::{GuildCreatePayload, GuildDeletePayload},

        interaction::{
            ApplicationCommandInteraction,
            ApplicationCommandInteractionContext,
            ResponsePayload,
            UpdateResponsePayload,
        },
        Result,
    },
    utility::message::get_invite_codes,
};

pub async fn handle_event(
    context: Arc<Context>,
    event: Event,
) -> Result<()> {
    match event {
        Event::ChannelCreate(payload) => context.cache.insert_channel(payload.0),
        Event::ChannelDelete(payload) => {
            let channel_id = payload.id;

            if let Some(guild_id) = payload.guild_id {
                context.cache.remove_channel(payload.id);
                context
                    .database
                    .remove_category_channel(guild_id, channel_id)
                    .await?;
                context
                    .database
                    .remove_ignored_channel(guild_id, channel_id)
                    .await?;
                context
                    .database
                    .remove_results_channel(guild_id, channel_id)
                    .await?;
                context.database.remove_channel_messages(channel_id).await?;
            }
        }
        Event::ChannelUpdate(payload) => context.cache.insert_channel(payload.0),
        Event::GuildCreate(payload) => {
            let guild_id = payload.id;
            let invite_check_category_ids =
                if let Some(guild) = context.database.get_guild(guild_id).await {
                    guild.category_channel_ids
                } else {
                    context
                        .database
                        .insert_guild_create_event(GuildCreatePayload {
                            guild_id: guild_id.get() as i64,
                        })
                        .await?;

                    HashSet::new()
                };

            context.database.insert_guild(guild_id).await?;

            let (communication_disabled_until, role_ids) = payload
                .0
                .members
                .into_iter()
                .find(|member| member.user.id.eq(&context.application_id.cast()))
                .map_or((None, HashSet::new()), |member| {
                    (
                        member
                            .communication_disabled_until
                            .map_or(None, |timestamp| {
                                Some(
                                    OffsetDateTime::from_unix_timestamp(timestamp.as_secs())
                                        .unwrap(),
                                )
                            }),
                        HashSet::from_iter(member.roles),
                    )
                });

            context.cache.insert_guild(
                payload.0.channels,
                guild_id,
                false,
                invite_check_category_ids,
                payload.0.name,
                payload.0.roles,
            );
            context.cache.insert_current_user(
                guild_id,
                communication_disabled_until,
                context.application_id.cast(),
                role_ids,
            );
        }
        Event::GuildDelete(payload) => {
            let guild_id = payload.id;

            context.cache.remove_guild(guild_id, payload.unavailable);
            context.database.remove_guild(guild_id).await?;
            context.database.remove_guild_messages(guild_id).await?;
            context
                .database
                .insert_guild_delete_event(GuildDeletePayload {
                    guild_id: guild_id.get() as i64,
                })
                .await?;
        }
        Event::GuildUpdate(payload) => {
            context
                .cache
                .update_guild(payload.id, None, None, Some(payload.name.clone()))
        }
        Event::InteractionCreate(payload) => {
            let Interaction {
                channel,
                data,
                guild_id,
                id,
                token,
                ..
            } = payload.0;

            let interaction_context = ApplicationCommandInteractionContext {
                id,
                interaction_client: context.interaction_client(),
                token,
            };
            let embed_builder = EmbedBuilder::new().color(0xF8F8FF);
            let channel_id = match channel {
                Some(channel) if vec![ChannelType::GuildAnnouncement, ChannelType::GuildText].contains(&channel.kind) => channel.id,
                _ => return interaction_context.respond(ResponsePayload {
                    embeds: vec![embed_builder.description("Sakura's commands may only be run in either announcement or text channels.".to_owned()).build()],
                    ephemeral: true,
                    ..Default::default()
                })
                .await
            };

            if !context.cache.has_minimum_channel_permissions(channel_id) {
                return interaction_context.respond(ResponsePayload {
                    embeds: vec![embed_builder.description("Sakura requires the **Embed Links**, **Read Message History**, **Send Messages**, and **View Channels** permissions in this channel.".to_owned()).build()],
                    ephemeral: true,
                    ..Default::default()
                })
                .await;
            }

            let Some(guild_id) = guild_id else {
                return interaction_context.respond(ResponsePayload {
                    embeds: vec![embed_builder.description("Sakura only works in guilds.".to_owned()).build()],
                    ephemeral: true,
                    ..Default::default()
                })
                .await
            };

            if context.cache.get_guild(guild_id).is_none() {
                return interaction_context
                    .respond(ResponsePayload {
                        embeds: vec![embed_builder
                            .description("Please kick and re-invite Sakura.".to_owned())
                            .build()],
                        ephemeral: true,
                        ..Default::default()
                    })
                    .await;
            }

            let Some(InteractionData::ApplicationCommand(data)) = data else {
                return interaction_context.respond(ResponsePayload {
                    embeds: vec![embed_builder.description("I have received an unknown interaction.".to_owned()).build()],
                    ephemeral: true,
                    ..Default::default()
                })
                .await
            };
            let mut interaction = ApplicationCommandInteraction {
                channel_id,
                context: interaction_context,
                data,
                guild_id,
            };
            let command_name = take(&mut interaction.data.name);
            let command_result = match command_name.as_str() {
                "Check message" => CheckMessageCommand::run(&context, &mut interaction).await,
                "check" => CheckCommand::run(&context, &interaction).await,
                "config" => ConfigCommand::run(&context, &mut interaction).await,
                "counts" => CountsCommand::run(&context, &interaction).await,
                "info" => InfoCommand::run(&context, &mut interaction).await,
                _ => {
                    return interaction
                        .context
                        .update_response(UpdateResponsePayload {
                            embeds: vec![embed_builder
                                .description(
                                    "I have received an unknown command with the name \"{}\".",
                                )
                                .build()],
                            ..Default::default()
                        })
                        .await
                }
            };

            if let Err(error) = command_result {
                return interaction
                    .context
                    .update_response(UpdateResponsePayload {
                        embeds: vec![embed_builder.description(error.to_string()).build()],
                        ..Default::default()
                    })
                    .await;
            }
        }
        Event::MemberUpdate(payload) => {
            if payload.user.id.eq(&context.application_id.cast()) {
                let guild_id = payload.guild_id;
                let communication_disabled_until =
                    payload
                        .communication_disabled_until
                        .map_or(None, |timestamp| {
                            Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs()).unwrap())
                        });
                let role_ids = HashSet::from_iter(payload.roles);

                context.cache.insert_current_user(
                    guild_id,
                    communication_disabled_until,
                    context.application_id.cast(),
                    role_ids,
                )
            }
        }
        Event::MessageCreate(payload) => {
            if let Some(guild_id) = payload.guild_id {
                if let Some(channel) = context.cache.get_channel(payload.channel_id) {
                    if let Some(parent_id) = channel.parent_id {
                        if let Some(guild) = context.cache.get_guild(guild_id) {
                            if guild.invite_check_category_ids.read().contains(&parent_id) {
                                let invite_codes =
                                    get_invite_codes(payload.0.content, payload.0.embeds);

                                for invite_code in invite_codes.clone() {
                                    context
                                        .database
                                        .insert_unchecked_invite(&invite_code)
                                        .await?;
                                }

                                context
                                    .database
                                    .insert_message(
                                        guild_id,
                                        payload.0.channel_id,
                                        payload.0.id,
                                        parent_id,
                                        invite_codes,
                                    )
                                    .await?;
                            }
                        }
                    }
                }
            }
        }
        Event::MessageDelete(payload) => context.database.remove_messages(vec![payload.id]).await?,
        Event::MessageDeleteBulk(payload) => context.database.remove_messages(payload.ids).await?,
        Event::MessageUpdate(payload) => {
            if let Some(guild_id) = payload.guild_id {
                if let Some(channel) = context.cache.get_channel(payload.channel_id) {
                    if let Some(parent_id) = channel.parent_id {
                        if let Some(guild) = context.cache.get_guild(guild_id) {
                            if guild.invite_check_category_ids.read().contains(&parent_id) {
                                let content = payload.content.unwrap_or_default();
                                let embeds = payload.embeds.unwrap_or_default();
                                let invite_codes = get_invite_codes(content, embeds);

                                for invite_code in invite_codes.clone() {
                                    context
                                        .database
                                        .insert_unchecked_invite(&invite_code)
                                        .await?;
                                }

                                context
                                    .database
                                    .insert_message(
                                        guild_id,
                                        payload.channel_id,
                                        payload.id,
                                        parent_id,
                                        invite_codes,
                                    )
                                    .await?;
                            }
                        }
                    }
                }
            }
        }
        Event::Ready(payload) => {
            for unvailable_guild in payload.guilds.into_iter() {
                context.cache.insert_unavailable_guild(unvailable_guild.id);
            }

            println!(
                "{}#{} is online!",
                payload.user.name, payload.user.discriminator
            );
        }
        Event::RoleCreate(payload) => context.cache.insert_role(payload.guild_id, payload.role),
        Event::RoleDelete(payload) => context.cache.remove_role(payload.role_id),
        Event::RoleUpdate(payload) => context.cache.insert_role(payload.guild_id, payload.role),
        Event::UnavailableGuild(payload) => context.cache.insert_unavailable_guild(payload.id),
        _ => {}
    }

    Ok(())
}
