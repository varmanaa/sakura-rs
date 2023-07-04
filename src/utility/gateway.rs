use std::collections::HashMap;

use twilight_gateway::{
    stream,
    Config,
    ConfigBuilder,
    EventTypeFlags,
    Intents,
    Session,
    Shard,
    ShardId,
};
use twilight_http::Client;

use crate::{types::Result, utility::constants::BOT_TOKEN};

pub async fn connect(
    client: &Client,
    current_sessions: HashMap<u64, Session>,
) -> Result<Vec<Shard>> {
    let intents = Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
    let event_types = EventTypeFlags::CHANNEL_CREATE
        | EventTypeFlags::CHANNEL_DELETE
        | EventTypeFlags::CHANNEL_UPDATE
        | EventTypeFlags::GATEWAY_HEARTBEAT
        | EventTypeFlags::GATEWAY_HEARTBEAT_ACK
        | EventTypeFlags::GATEWAY_HELLO
        | EventTypeFlags::GATEWAY_INVALIDATE_SESSION
        | EventTypeFlags::GATEWAY_RECONNECT
        | EventTypeFlags::GUILD_CREATE
        | EventTypeFlags::GUILD_DELETE
        | EventTypeFlags::INTERACTION_CREATE
        | EventTypeFlags::MEMBER_UPDATE
        | EventTypeFlags::MESSAGE_CREATE
        | EventTypeFlags::MESSAGE_DELETE
        | EventTypeFlags::MESSAGE_DELETE_BULK
        | EventTypeFlags::MESSAGE_UPDATE
        | EventTypeFlags::READY
        | EventTypeFlags::ROLE_CREATE
        | EventTypeFlags::ROLE_DELETE
        | EventTypeFlags::ROLE_UPDATE
        | EventTypeFlags::UNAVAILABLE_GUILD;
    let config = Config::builder(BOT_TOKEN.to_owned(), intents)
        .event_types(event_types)
        .build();
    let per_shard_config = |shard_id: ShardId, builder: ConfigBuilder| {
        match current_sessions.get(&shard_id.number()) {
            None => builder.build(),
            Some(session) => builder.session(session.to_owned()).build(),
        }
    };
    let shards = stream::create_recommended(client, config, per_shard_config)
        .await?
        .collect::<Vec<Shard>>();

    Ok(shards)
}

pub async fn reconnect(
    client: &Client,
    shards: &mut Vec<Shard>,
) -> Result<()> {
    *shards = connect(client, HashMap::default()).await?;

    Ok(())
}
