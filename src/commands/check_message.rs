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
        let Some(message) = interaction.message() else {
            return Err(Error::Custom("I could not find a message.".to_owned()))
        };
        let Some(channel) = context.cache.get_channel(message.channel_id) else {
            return Err(Error::Custom(
                "Sakura only looks at messages in announcement and text channels.".to_owned(),
            ))
        };
        let Some(parent_id) = channel.parent_id else {
            return Err(Error::Custom(
                "Please ensure this message is within a category before checking it."
                    .to_owned(),
            ))
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
                "Please ensure this message is within an **added** category before checking it."
                    .to_owned(),
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
                "Sakura found some invites and will add them to the next invite check.".to_owned(),
            )
            .build();

        interaction
            .context
            .update_response(UpdateResponsePayload {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
