mod commands;
mod events;
mod structs;
mod tasks;
mod types;
mod utility;

use std::{collections::HashMap, sync::Arc};

use dotenv::dotenv;
use futures::StreamExt;
use twilight_gateway::{error::ReceiveMessageErrorType, stream::ShardEventStream};
use twilight_http::Client;
use twilight_model::gateway::CloseCode;

use crate::{
    types::{cache::Cache, context::Context, database::Database},
    utility::{
        constants::BOT_TOKEN,
        gateway::{connect, reconnect},
    },
};

#[tokio::main]
async fn main() -> types::Result<()> {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let http = Client::new(BOT_TOKEN.to_owned());
    let application_id = http.current_user_application().await?.model().await?.id;
    let cache = Cache::new();
    let database = Database::new()?;
    let mut shards = connect(&http, HashMap::default()).await?;
    let context = Arc::new(Context::new(application_id, cache, database, http));

    context.database.create_tables().await?;

    let commands = commands::get_commands();

    context
        .interaction_client()
        .set_global_commands(&commands)
        .await?;

    let task_context = Arc::clone(&context);

    tokio::spawn(async move {
        tasks::handle_tasks(task_context).await.unwrap();
    });

    'outer: loop {
        let mut stream = ShardEventStream::new(shards.iter_mut());

        'inner: loop {
            let error = match stream.next().await {
                None => return Ok(()),
                Some((_, Err(error))) => {
                    tracing::info!(?error, "Error receiving event");

                    error
                }
                Some((shard_ref, Ok(event))) => {
                    tracing::info!("Received event - {:#?}", event.kind());
                    tracing::info!("Shard status - {:#?}", shard_ref.status());

                    let shard_id = shard_ref.id().number();

                    context
                        .latencies
                        .write()
                        .insert(shard_id, Arc::new(shard_ref.latency().clone()));

                    let event_context = Arc::clone(&context);

                    tokio::spawn(async move {
                        events::handle_event(event_context, shard_id, event)
                            .await
                            .unwrap()
                    });

                    continue 'inner;
                }
            };
            let should_reconnect = matches!(
                error.kind(),
                ReceiveMessageErrorType::FatallyClosed {
                    close_code: CloseCode::ShardingRequired | CloseCode::UnknownError
                }
            );

            if should_reconnect {
                drop(stream);

                reconnect(&context.http, &mut shards).await?;

                continue 'outer;
            }
            if error.is_fatal() {
                return Ok(());
            }
        }
    }
}
