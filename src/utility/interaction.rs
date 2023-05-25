use twilight_model::{
    application::interaction::Interaction,
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::types::{context::Context, Result};

pub async fn respond_with_ephemeral_interaction_response(
    context: &Context,
    interaction: &Interaction,
    description: String,
) -> Result<()> {
    let embed = EmbedBuilder::new()
        .color(0xF8F8FF)
        .description(description)
        .build();

    context
        .interaction_client()
        .create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                data: Some(InteractionResponseData {
                    components: None,
                    embeds: Some(vec![embed]),
                    flags: Some(MessageFlags::EPHEMERAL),
                    ..Default::default()
                }),
                kind: InteractionResponseType::ChannelMessageWithSource,
            },
        )
        .await?;

    Ok(())
}
