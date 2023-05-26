use std::{collections::HashSet, mem::take, sync::Arc};

use time::OffsetDateTime;
use twilight_gateway::Event;
use twilight_model::application::interaction::InteractionData;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    commands::{check_message::CheckMessageCommand, info::InfoCommand, latency::LatencyCommand},
    types::{
        context::Context,
        database::{GuildCreatePayload, GuildDeletePayload},
        interaction::{ApplicationCommandInteraction, ResponsePayload},
        Result,
    },
    utility::{
        interaction::respond_with_ephemeral_interaction_response,
        message::get_invite_codes,
    },
};

pub async fn handle_event(
    event: Event,
    context: Arc<Context>,
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
            context
                .cache
                .insert_current_user(guild_id, communication_disabled_until, role_ids);
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
            let interaction_client = context.interaction_client();
            let interaction_payload = payload.0;
            let guild_id = match interaction_payload.guild_id {
                Some(guild_id) => guild_id,
                None => {
                    respond_with_ephemeral_interaction_response(
                        &context,
                        &interaction_payload,
                        "Sakura-RS only works in guilds.".to_owned(),
                    )
                    .await?;

                    return Ok(());
                }
            };

            if context.cache.get_guild(guild_id).is_none() {
                respond_with_ephemeral_interaction_response(
                    &context,
                    &interaction_payload,
                    "Please kick and re-invite Sakura-RS.".to_owned(),
                )
                .await?;

                return Ok(());
            }

            match interaction_payload.data {
                Some(InteractionData::ApplicationCommand(data)) => {
                    let mut interaction = ApplicationCommandInteraction {
                        channel_id: interaction_payload.channel.unwrap().id,
                        data,
                        guild_id,
                        id: interaction_payload.id,
                        interaction_client,
                        token: interaction_payload.token,
                    };
                    let command_name = take(&mut interaction.data.name);

                    match command_name.as_str() {
                        "Check message" => CheckMessageCommand::run(&context, interaction).await?,
                        "info" => InfoCommand::run(&context, interaction).await?,
                        "latency" => LatencyCommand::run(&context, interaction).await?,
                        name => {
                            let embed = EmbedBuilder::new()
                                .color(0xF8F8FF)
                                .description(format!(
                                    "I have received an unknown command with the name \"{name}\"."
                                ))
                                .build();
                            let payload = ResponsePayload {
                                components: None,
                                embeds: Some(vec![embed]),
                                ephemeral: true,
                            };

                            interaction.respond(payload).await?;
                        }
                    }
                }
                _ => {
                    respond_with_ephemeral_interaction_response(
                        &context,
                        &interaction_payload,
                        "I have received an unknown interaction.".to_owned(),
                    )
                    .await?;

                    return Ok(());
                }
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

                context
                    .cache
                    .insert_current_user(guild_id, communication_disabled_until, role_ids)
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
