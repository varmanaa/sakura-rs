mod events;
mod structs;
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
use types::{cache::Cache, context::Context, database::Database, Result};

#[tokio::main]
async fn main() -> Result<()> {
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
    let cache = Cache::new();
    let database = Database::new()?;

    database.create_tables().await?;

    let context = Arc::new(Context::new(application_id, cache, database, http));

    loop {
        let (_shard, event) = match stream.next().await {
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

        events::handle_event(event, event_context).await?;
    }

    Ok(())
}
