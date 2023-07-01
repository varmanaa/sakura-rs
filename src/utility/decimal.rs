use crate::utility::constants::COMMA_REGEX;

pub fn add_commas(decimal: String) -> String {
    COMMA_REGEX.replace_all(decimal.as_str(), "$1,").to_string()
}
