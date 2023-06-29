mod commands;
mod events;
mod structs;
mod tasks;
mod types;
mod utility;

use std::{env, sync::Arc};

use dotenv::dotenv;
use futures::StreamExt;
use twilight_gateway::{
    stream::{self, ShardEventStream},
    Config,
    Intents,
};
use twilight_http::Client;
use utility::constants::DEVELOPMENT_GUILD_ID;

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

    #[cfg(feature = "production")]
    {
        context
            .interaction_client()
            .set_global_commands(&commands)
            .await?;

        if context.cache.get_guild(*DEVELOPMENT_GUILD_ID).is_some() {
            context
                .interaction_client()
                .set_guild_commands(*DEVELOPMENT_GUILD_ID, &[])
                .await?;
        }
    }

    #[cfg(not(feature = "production"))]
    {
        context
            .interaction_client()
            .set_global_commands(&[])
            .await?;

        if context.cache.get_guild(*DEVELOPMENT_GUILD_ID).is_some() {
            context
                .interaction_client()
                .set_guild_commands(*DEVELOPMENT_GUILD_ID, &commands)
                .await?;
        }
    }

    let task_context = context.clone();

    tokio::spawn(async move {
        tasks::handle_tasks(task_context).await.unwrap();
    });

    loop {
        let (shard, event) = match stream.next().await {
            Some((shard, Ok(event))) => (shard, event),
            Some((_shard, Err(source))) => {
                if source.is_fatal() {
                    break;
                }

                continue;
            }
            None => break,
        };
        let event_context = context.clone();
        let latency = shard.latency().clone();

        tokio::spawn(async move {
            events::handle_event(latency, event, event_context)
                .await
                .unwrap();
        });
    }

    Ok(())
}
