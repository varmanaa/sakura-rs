use std::sync::Arc;

use twilight_model::gateway::payload::incoming::MessageDelete;

use crate::types::{context::Context, Result};

pub async fn handle_message_delete(
    context: Arc<Context>,
    payload: MessageDelete,
) -> Result<()> {
    context.database.remove_messages(vec![payload.id]).await?;

    Ok(())
}
