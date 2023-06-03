use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::{command::CommandBuilder, embed::EmbedBuilder};

use crate::{
    types::{
        context::Context,
        interaction::{
            ApplicationCommandInteraction,
            DeferInteractionPayload,
            UpdateResponsePayload,
        },
        Result,
    },
    utility::message::get_invite_codes,
};

pub struct CheckMessageCommand {}

impl CheckMessageCommand {
    pub fn create_command() -> Command {
        CommandBuilder::new("Check message", "", CommandType::Message).build()
    }

    pub async fn run(
        context: &Context,
        mut interaction: ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let embed_builder = EmbedBuilder::new().color(0xF8F8FF);
        let message = match interaction.message() {
            Some(message) => message,
            None => {
                let embed = embed_builder
                    .description("I could not find a message.".to_owned())
                    .build();

                interaction
                    .update_response(UpdateResponsePayload {
                        embeds: Some(&[embed]),
                    })
                    .await?;

                return Ok(());
            }
        };
        let channel = match context.cache.get_channel(message.channel_id) {
            Some(channel) => channel,
            None => {
                let embed = embed_builder
                    .description(
                        "Sakura-RS only looks at messages in announcement and text channels."
                            .to_owned(),
                    )
                    .build();

                interaction
                    .update_response(UpdateResponsePayload {
                        embeds: Some(&[embed]),
                    })
                    .await?;

                return Ok(());
            }
        };
        let parent_id = match channel.parent_id {
            Some(parent_id) => parent_id,
            None => {
                let embed = embed_builder
                    .description(
                        "Please ensure this message is within a category before checking it."
                            .to_owned(),
                    )
                    .build();

                interaction
                    .update_response(UpdateResponsePayload {
                        embeds: Some(&[embed]),
                    })
                    .await?;

                return Ok(());
            }
        };
        let is_within_added_category = match context.cache.get_guild(interaction.guild_id) {
            Some(cached_guild) => {
                cached_guild
                    .invite_check_category_ids
                    .read()
                    .contains(&parent_id)
            }
            None => false,
        };

        if !is_within_added_category {
            let embed = embed_builder
            .description(
                "Please ensure this message is within an **added** category before checking it."
                    .to_owned(),
            )
            .build();

            interaction
                .update_response(UpdateResponsePayload {
                    embeds: Some(&[embed]),
                })
                .await?;

            return Ok(());
        }

        let invite_codes = get_invite_codes(message.content, message.embeds);

        if invite_codes.is_empty() {
            let embed = embed_builder
                .description("No invite codes found.".to_owned())
                .build();

            interaction
                .update_response(UpdateResponsePayload {
                    embeds: Some(&[embed]),
                })
                .await?;

            return Ok(());
        }

        for invite_code in invite_codes.iter() {
            context
                .database
                .insert_unchecked_invite(&invite_code)
                .await?;
        }

        context
            .database
            .insert_message(
                interaction.guild_id,
                channel.channel_id,
                message.id,
                parent_id,
                invite_codes,
            )
            .await?;

        let embed = embed_builder
            .description(
                "Sakura-RS found some invites and will add them to the next invite check."
                    .to_owned(),
            )
            .build();

        interaction
            .update_response(UpdateResponsePayload {
                embeds: Some(&[embed]),
            })
            .await?;

        Ok(())
    }
}
