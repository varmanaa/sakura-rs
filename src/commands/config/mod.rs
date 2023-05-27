mod add_category_channel;
mod add_ignored_channel;
mod remove_category_channel;
mod remove_ignored_channel;
mod set_embed_color;
mod set_results_channel;
mod show;

use twilight_interactions::command::{CommandModel, CreateCommand};

use self::{
    add_category_channel::ConfigAddCategoryChannelCommand,
    add_ignored_channel::ConfigAddIgnoredChannelCommand,
    remove_category_channel::ConfigRemoveCategoryChannelCommand,
    remove_ignored_channel::ConfigRemoveIgnoredChannelCommand,
    set_embed_color::ConfigSetEmbedColorCommand,
    set_results_channel::ConfigSetResultsChannelCommand,
    show::ConfigShowCommand,
};
use crate::types::{context::Context, interaction::ApplicationCommandInteraction, Result};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Manage configuration for Sakura-RS", name = "config")]
pub enum ConfigCommand {
    #[command(name = "add-category-channel")]
    AddCategoryChannel(ConfigAddCategoryChannelCommand),
    #[command(name = "add-ignored-channel")]
    AddIgnoredChannel(ConfigAddIgnoredChannelCommand),
    #[command(name = "remove-category-channel")]
    RemoveCategoryChannel(ConfigRemoveCategoryChannelCommand),
    #[command(name = "remove-ignored-channel")]
    RemoveIgnoredChannel(ConfigRemoveIgnoredChannelCommand),
    #[command(name = "set-embed-color")]
    SetEmbedColor(ConfigSetEmbedColorCommand),
    #[command(name = "set-results-channel")]
    SetResultsChannel(ConfigSetResultsChannelCommand),
    #[command(name = "show")]
    Show(ConfigShowCommand),
}

impl ConfigCommand {
    pub async fn run(
        context: &Context,
        mut interaction: ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        match ConfigCommand::from_interaction(interaction.input_data())? {
            ConfigCommand::AddCategoryChannel(options) => {
                ConfigAddCategoryChannelCommand::run(context, interaction, options).await?
            }
            ConfigCommand::AddIgnoredChannel(options) => {
                ConfigAddIgnoredChannelCommand::run(context, interaction, options).await?
            }
            ConfigCommand::RemoveCategoryChannel(options) => {
                ConfigRemoveCategoryChannelCommand::run(context, interaction, options).await?
            }
            ConfigCommand::RemoveIgnoredChannel(options) => {
                ConfigRemoveIgnoredChannelCommand::run(context, interaction, options).await?
            }
            ConfigCommand::SetEmbedColor(options) => {
                ConfigSetEmbedColorCommand::run(context, interaction, options).await?
            }
            ConfigCommand::SetResultsChannel(options) => {
                ConfigSetResultsChannelCommand::run(context, interaction, options).await?
            }
            ConfigCommand::Show(options) => {
                ConfigShowCommand::run(context, interaction, options).await?
            }
        }

        Ok(())
    }
}
