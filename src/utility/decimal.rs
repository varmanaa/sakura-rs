use crate::utility::constants::COMMA_REGEX;

pub fn add_commas(decimal: f64) -> String {
    COMMA_REGEX
        .replace_all(decimal.to_string().as_str(), "$1,")
        .to_string()
}