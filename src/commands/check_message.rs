use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::{command::CommandBuilder, embed::EmbedBuilder};

use crate::{
    types::{
        context::Context,
        interaction::{ApplicationCommandInteraction, UpdateResponsePayload},
        Result,
    },
    utility::{error::Error, message::get_invite_codes},
};

pub struct CheckMessageCommand {}

impl CheckMessageCommand {
    pub fn create_command() -> Command {
        CommandBuilder::new("Check message", "", CommandType::Message).build()
    }

    pub async fn run(
        context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        match interaction.message() {
            None => return Err(Error::Custom("I could not find a message.".to_owned())),
            Some(message) => {
                let channel =
                    match context.cache.get_channel(message.channel_id) {
                        None => return Err(Error::Custom(
                            "Sakura-RS only looks at messages in announcement and text channels."
                                .to_owned(),
                        )),
                        Some(channel) => channel,
                    };
                let parent_id =
                    match channel.parent_id {
                        None => return Err(Error::Custom(
                            "Please ensure this message is within a category before checking it."
                                .to_owned(),
                        )),
                        Some(parent_id) => parent_id,
                    };

                if !context
                    .cache
                    .get_guild(interaction.guild_id)
                    .map_or(false, |cached_guild| {
                        cached_guild
                            .invite_check_category_ids
                            .read()
                            .contains(&parent_id)
                    })
                {
                    return Err(Error::Custom(
                        "Please ensure this message is within an **added** category before checking it.".to_owned(),
                    ));
                }

                let invite_codes = get_invite_codes(message.content, message.embeds);

                if invite_codes.is_empty() {
                    return Err(Error::Custom("No invite codes found.".to_owned()));
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

                let embed = EmbedBuilder::new()
                    .color(0xF8F8FF)
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
    }
}
