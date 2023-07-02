use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    types::{
        cache::GuildUpdate,
        context::Context,
        interaction::{
            ApplicationCommandInteraction,
            DeferInteractionPayload,
            UpdateResponsePayload,
        },
        Result,
    },
    utility::error::Error,
};

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Remove a channel from the list of ignored channels",
    name = "remove-ignored-channel"
)]
pub struct ConfigRemoveIgnoredChannelCommand {
    #[command(
        channel_types = "guild_news guild_text",
        desc = "The announcement or text channel"
    )]
    channel: Id<ChannelMarker>,
}

impl ConfigRemoveIgnoredChannelCommand {
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

        let Some(database_guild) = context.database.get_guild(interaction.guild_id).await else {
            return Err(Error::Custom(
                "Please kick and re-invite Sakura.".to_owned(),
            ))
        };
        let channel_id = options.channel;

        if !database_guild.ignored_channel_ids.contains(&channel_id) {
            return Err(Error::Custom(format!(
                "<#{channel_id}> is not an ignored channel."
            )));
        }

        let updated_category_channel_ids = context
            .database
            .remove_channel(interaction.guild_id, channel_id)
            .await?;

        context.cache.update_guild(
            interaction.guild_id,
            GuildUpdate {
                in_check: Some(false),
                invite_check_category_ids: Some(updated_category_channel_ids),
                ..Default::default()
            },
        );

        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(format!(
                "<#{channel_id}> will no longer be ignored during invite checks."
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
