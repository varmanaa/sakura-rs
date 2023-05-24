use std::{borrow::Cow, mem::take};

use twilight_interactions::command::CommandInputData;
use twilight_model::{
    application::command::CommandType,
    channel::{message::MessageFlags, Message},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::types::{
    interaction::{
        ApplicationCommandInteraction,
        DeferInteractionPayload,
        ResponsePayload,
        UpdateResponsePayload,
    },
    Result,
};

impl ApplicationCommandInteraction<'_> {
    pub async fn defer(
        &self,
        payload: DeferInteractionPayload,
    ) -> Result<()> {
        let response = InteractionResponse {
            data: Some(InteractionResponseData {
                flags: payload.ephemeral.then(|| MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
        };

        self.interaction_client
            .create_response(self.id, &self.token, &response)
            .await?;

        Ok(())
    }

    pub fn input_data(&mut self) -> CommandInputData {
        CommandInputData {
            options: take(&mut self.data.options),
            resolved: self.data.resolved.take().map(Cow::Owned),
        }
    }

    pub fn message(&mut self) -> Option<Message> {
        match self.data.kind {
            CommandType::Message => {
                match self.input_data().resolved {
                    Some(resolved) => resolved.messages.values().next().cloned(),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub async fn respond(
        &self,
        payload: ResponsePayload,
    ) -> Result<()> {
        let response = InteractionResponse {
            data: Some(InteractionResponseData {
                components: payload.components,
                embeds: payload.embeds,
                flags: payload.ephemeral.then(|| MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
            kind: InteractionResponseType::ChannelMessageWithSource,
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

    pub async fn update_response(
        &self,
        payload: UpdateResponsePayload<'_>,
    ) -> Result<()> {
        self.interaction_client
            .update_response(&self.token)
            .embeds(payload.embeds)?
            .await?;

        Ok(())
    }
}
