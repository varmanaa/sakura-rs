use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle, Component};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::types::{
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdateResponsePayload},
    Result,
};

#[derive(CommandOption, CreateOption)]
enum Query {
    #[option(name = "Links", value = "links")]
    Links,
    #[option(name = "Setup", value = "setup")]
    Setup,
    #[option(name = "Source code", value = "source-code")]
    SourceCode,
}

#[allow(dead_code)]
#[derive(CommandModel, CreateCommand)]
#[command(desc = "Search information on Sakura", name = "info")]
pub struct InfoCommand {
    #[command(desc = "The topic to search")]
    query: Query,
}

impl InfoCommand {
    pub async fn run(
        _context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let options = InfoCommand::from_interaction(interaction.input_data())?;
        let (components, embeds) = match options.query {
            Query::Links => {
                let components = vec![Component::ActionRow(ActionRow {
                    components: vec![
                        Component::Button(Button {
                            custom_id: None,
                            disabled: false,
                            emoji: None,
                            label: Some("Privacy Policy".to_owned()),
                            style: ButtonStyle::Link,
                            url: Some(
                                "https://github.com/varmanaa/sakura-rs/blob/main/docs/PRIVACY_POLICY.md".to_owned(),
                            ),
                        }),
                        Component::Button(Button {
                            custom_id: None,
                            disabled: false,
                            emoji: None,
                            label: Some("Support Server".to_owned()),
                            style: ButtonStyle::Link,
                            url: Some(
                                "https://discord.gg/wtZurTFJdH".to_owned(),
                            ),
                        }),
                        Component::Button(Button {
                            custom_id: None,
                            disabled: false,
                            emoji: None,
                            label: Some("Terms of Service".to_owned()),
                            style: ButtonStyle::Link,
                            url: Some(
                                "https://github.com/varmanaa/sakura-rs/blob/main/docs/TERMS_OF_SERVICE.md".to_owned(),
                            ),
                        }),
                    ],
                })];
                let embeds = vec![EmbedBuilder::new()
                    .color(0xF8F8FF)
                    .description("Click/tap the button that interests you!")
                    .title("Documents")
                    .build()];

                (components, embeds)
            }
            Query::Setup => {
                let embeds = vec![EmbedBuilder::new()
                    .color(0xF8F8FF)
                    .field(
                        EmbedFieldBuilder::new(
                            "Permissions",
                            "
                                For (non-administrator) users, please enable the **Use Application Commands** permission \
                                (as Sakura only uses slash commands). For Sakura, please enable the **Embed Links**, \
                                **Read Message History**, **Send Messages**, and **View Channels** permissions in the \
                                categories and channels that need to be checked.
                            "
                        ).build()
                    )
                    .field(
                        EmbedFieldBuilder::new(
                            "Setup",
                            vec![
                                "- Set a channel to send invite check results in using the `/config set-results-channel` command.",
                                "- Add categories to check using the `/config add-category-channel` command.",
                                "- Add channels to ignore using the `/config add-ignored-channel` command.",
                                "- Run an invite check using the `/check` command."
                            ].join("\n")
                        ).build()
                    )
                    .title("Sakura 101")
                    .build()];

                (Vec::new(), embeds)
            }
            Query::SourceCode => {
                let components = vec![Component::ActionRow(ActionRow {
                    components: vec![Component::Button(Button {
                        custom_id: None,
                        disabled: false,
                        emoji: None,
                        label: Some("Source code".to_owned()),
                        style: ButtonStyle::Link,
                        url: Some("https://github.com/varmanaa/sakura-rs".to_owned()),
                    })],
                })];
                let embeds = vec![EmbedBuilder::new()
                    .color(0xF8F8FF)
                    .description("Click/tap the button below!")
                    .title("Source code")
                    .build()];

                (components, embeds)
            }
        };

        interaction
            .context
            .update_response(UpdateResponsePayload {
                components,
                embeds,
            })
            .await?;

        Ok(())
    }
}
