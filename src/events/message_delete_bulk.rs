use std::sync::Arc;

use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

use crate::types::{context::Context, Result};

pub async fn handle_message_delete_bulk(
    context: Arc<Context>,
    payload: MessageDeleteBulk,
) -> Result<()> {
    context.database.remove_messages(payload.ids).await?;

    Ok(())
}
