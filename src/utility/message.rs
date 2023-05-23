use std::collections::HashSet;

use twilight_model::channel::message::Embed;

use crate::utility::constants::DISCORD_INVITE_REGEX;

pub fn get_invite_codes(
    content: String,
    embeds: Vec<Embed>,
) -> HashSet<String> {
    let mut message_strings = embeds
        .into_iter()
        .map(|embed| embed.description.unwrap_or_default())
        .collect::<Vec<String>>();

    message_strings.push(content);

    let message_string = message_strings.join(" ");
    let mut invite_codes = HashSet::new();

    for captures_result in DISCORD_INVITE_REGEX.captures_iter(&message_string) {
        if let Ok(captures) = captures_result {
            if let Some(code) = captures.get(1) {
                invite_codes.insert(code.as_str().to_owned());
            }
        }
    }

    invite_codes
}
