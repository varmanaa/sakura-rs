use std::time::Duration;

use tokio::time::sleep;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

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
    utility::{error::Error, message::get_invite_codes},
};

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Add a category for Sakura to check",
    name = "add-category-channel"
)]
pub struct ConfigAddCategoryChannelCommand {
    #[command(channel_types = "guild_category", desc = "The category channel")]
    channel: Id<ChannelMarker>,
}

impl ConfigAddCategoryChannelCommand {
    pub async fn run(
        context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
        options: Self,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let Some(cached_guild) = context.cache.get_guild(interaction.guild_id) else {
            return Err(Error::Custom(
                "Please kick and re-invite Sakura.".to_owned(),
            ))
        };
        let category_id = options.channel;

        if cached_guild
            .invite_check_category_ids
            .read()
            .contains(&category_id)
        {
            return Err(Error::Custom(format!(
                "<#{category_id}> is already an added category."
            )));
        }

        if cached_guild.in_check {
            return Err(Error::Custom(
                "Sakura is either running an invite check or adding a category at the
        moment. Please wait until this is done before trying again."
                    .to_owned(),
            ));
        }

        let mut channel_ids_to_process = Vec::new();
        let mut invisible_channels = Vec::new();

        for channel_id in cached_guild.channel_ids.read().clone().into_iter() {
            let channel = match context.cache.get_channel(channel_id) {
                None => continue,
                Some(channel) => channel,
            };

            if channel
                .parent_id
                .map_or(true, |parent_id| !parent_id.eq(&category_id))
            {
                continue;
            }

            if context.cache.has_minimum_channel_permissions(channel_id) {
                channel_ids_to_process.push(channel_id)
            } else {
                invisible_channels.push(format!("- <#{channel_id}>"))
            }
        }

        if !invisible_channels.is_empty() {
            return Err(Error::Custom(format!("Sakura is unable to check the following channels:\n{}\nPlease give permission for Sakura to read these channels and add the category again.", invisible_channels.join("\n")),));
        }

        for channel_id in channel_ids_to_process {
            sleep(Duration::from_millis(1000)).await;

            let messages = context
                .http
                .channel_messages(channel_id)
                .limit(10)?
                .await?
                .model()
                .await?;

            for message in messages {
                sleep(Duration::from_millis(100)).await;

                let invite_codes = get_invite_codes(message.content, message.embeds);

                for invite_code in invite_codes.iter() {
                    sleep(Duration::from_millis(100)).await;

                    context
                        .database
                        .insert_unchecked_invite(invite_code)
                        .await?;
                }

                context
                    .database
                    .insert_message(
                        interaction.guild_id,
                        channel_id,
                        message.id,
                        category_id,
                        invite_codes,
                    )
                    .await?;
            }
        }

        let updated_category_channel_ids = context
            .database
            .insert_category_channel(interaction.guild_id, category_id)
            .await?;

        context.cache.update_guild(
            interaction.guild_id,
            Some(false),
            Some(updated_category_channel_ids),
            None,
        );

        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(format!(
                "<#{category_id}> will now be checked during invite checks."
            ))
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
