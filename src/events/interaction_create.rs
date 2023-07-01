use std::{mem::take, sync::Arc};

use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    channel::ChannelType,
    gateway::payload::incoming::InteractionCreate,
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    commands::{
        check::CheckCommand,
        check_message::CheckMessageCommand,
        config::ConfigCommand,
        counts::CountsCommand,
        info::InfoCommand,
        latency::LatencyCommand,
        stats::StatsCommand,
    },
    types::{
        context::Context,
        interaction::{
            ApplicationCommandInteraction,
            ApplicationCommandInteractionContext,
            ResponsePayload,
            UpdateResponsePayload,
        },
        Result,
    },
};

pub async fn handle_interaction_create(
    context: Arc<Context>,
    shard_id: u64,
    payload: InteractionCreate,
) -> Result<()> {
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
        shard_id,
    };
    let command_name = take(&mut interaction.data.name);
    let command_result = match command_name.as_str() {
        "Check message" => CheckMessageCommand::run(&context, &mut interaction).await,
        "check" => CheckCommand::run(&context, &interaction).await,
        "config" => ConfigCommand::run(&context, &mut interaction).await,
        "counts" => CountsCommand::run(&context, &interaction).await,
        "info" => InfoCommand::run(&context, &mut interaction).await,
        "latency" => LatencyCommand::run(&context, &interaction).await,
        "stats" => StatsCommand::run(&context, &interaction).await,
        _ => {
            return interaction
                .context
                .update_response(UpdateResponsePayload {
                    embeds: vec![embed_builder
                        .description("I have received an unknown command with the name \"{}\".")
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

    Ok(())
}
