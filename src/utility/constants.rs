use std::env;

use fancy_regex::Regex;
use once_cell::sync::Lazy;

pub static BOT_TOKEN: Lazy<String> = Lazy::new(|| env::var("BOT_TOKEN").unwrap());
pub static COMMA_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"/(\d)(?=(\d{3})+(?!\d))/g").unwrap());
pub static DATABASE_URL: Lazy<String> = Lazy::new(|| env::var("DATABASE_URL").unwrap());
pub static DISCORD_INVITE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)(?:https?:\/\/)?(?:\w+\.)?discord(?:(?:app)?\.com\/invite|\.gg)\/(?<code>[a-z0-9-]+)",
    )
    .unwrap()
});
