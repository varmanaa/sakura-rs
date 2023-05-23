use std::env;

use fancy_regex::Regex;
use once_cell::sync::Lazy;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| env::var("DATABASE_URL").unwrap());
pub static DISCORD_INVITE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)(?:https?:\/\/)?(?:\w+\.)?discord(?:(?:app)?\.com\/invite|\.gg)\/(?<code>[a-z0-9-]+)",
    )
    .unwrap()
});
