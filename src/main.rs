mod commands;
mod events;
mod structs;
mod tasks;
mod types;
mod utility;

use std::{env, sync::Arc};

use dotenv::dotenv;
use futures::StreamExt;
use time::OffsetDateTime;
use twilight_gateway::{
    stream::{self, ShardEventStream},
    Config,
    Intents,
};
use twilight_http::Client;
use types::context::Shard;

#[tokio::main]
async fn main() -> types::Result<()> {
    dotenv().ok();

    let token = env::var("BOT_TOKEN")?;
    let http = Arc::new(Client::new(token.clone()));
    let config = Config::new(
        token.clone(),
        Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );
    let mut shards = stream::create_recommended(&http, config, |_, builder| builder.build())
        .await?
        .collect::<Vec<_>>();
    let mut stream = ShardEventStream::new(shards.iter_mut());
    let application_id = http.current_user_application().await?.model().await?.id;
    let cache = types::cache::Cache::new();
    let database = types::database::Database::new()?;

    database.create_tables().await?;

    let context = Arc::new(types::context::Context::new(
        application_id,
        cache,
        database,
        http,
    ));
    let commands = commands::get_commands();

    context
        .interaction_client()
        .set_global_commands(&commands)
        .await?;

    let task_context = context.clone();

    tokio::spawn(async move {
        tasks::handle_tasks(task_context).await.unwrap();
    });

    loop {
        let (shard_ref, event) = match stream.next().await {
            None => break,
            Some((_, Err(source))) => {
                if source.is_fatal() {
                    break;
                }

                continue;
            }
            Some((shard_ref, Ok(event))) => (shard_ref, event),
        };
        let shard_id = shard_ref.id().number();
        let ready_at = if shard_ref.status().is_connected() {
            match context.shards.read().get(&shard_id) {
                Some(shard) if shard.ready_at.is_some() => shard.ready_at,
                _ => Some(OffsetDateTime::now_utc()),
            }
        } else {
            None
        };

        context.shards.write().insert(
            shard_id,
            Arc::new(Shard {
                latency: shard_ref.latency().clone(),
                ready_at,
                shard_id,
            }),
        );

        let event_context = context.clone();

        tokio::spawn(async move {
            events::handle_event(event_context, shard_id, event)
                .await
                .unwrap();
        });
    }

    Ok(())
}
