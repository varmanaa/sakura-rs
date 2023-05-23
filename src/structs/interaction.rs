use twilight_model::{
    channel::{
        message::{Embed, MessageFlags},
        Message,
    },
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::types::{interaction::ApplicationCommandInteraction, Result};

impl ApplicationCommandInteraction<'_> {
    pub async fn defer(
        &self,
        ephemeral: bool,
    ) -> Result<()> {
        let response = InteractionResponse {
            data: Some(InteractionResponseData {
                flags: ephemeral.then(|| MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
        };

        self.interaction_client
            .create_response(self.id, &self.token, &response)
            .await?;

        Ok(())
    }

    pub async fn response(&self) -> Result<Message> {
        let message = self
            .interaction_client
            .response(&self.token)
            .await?
            .model()
            .await?;

        Ok(message)
    }

    pub async fn update_response_with_embed(
        &self,
        embed: Embed,
    ) -> Result<()> {
        self.interaction_client
            .update_response(&self.token)
            .embeds(Some(&[embed]))?
            .await?;

        Ok(())
    }
}
