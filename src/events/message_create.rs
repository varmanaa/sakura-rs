use std::sync::Arc;

use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    types::{context::Context, Result},
    utility::message::get_invite_codes,
};

pub async fn handle_message_create(
    context: Arc<Context>,
    payload: MessageCreate,
) -> Result<()> {
    let Some(guild_id) = payload.guild_id else {
        return Ok(())
    };
    let Some(cached_guild) = context.cache.get_guild(guild_id) else {
        return Ok(())
    };
    let Some(channel) = context.cache.get_channel(payload.channel_id) else {
        return Ok(())
    };
    let Some(parent_id) = channel.parent_id else {
        return Ok(())
    };

    if cached_guild
        .invite_check_category_ids
        .read()
        .contains(&parent_id)
    {
        let invite_codes = get_invite_codes(payload.0.content, payload.0.embeds);

        for invite_code in invite_codes.iter() {
            context
                .database
                .insert_unchecked_invite(invite_code)
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

    Ok(())
}
