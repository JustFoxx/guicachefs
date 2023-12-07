mod language;

use crate::utils::language::{DEFAULT_LANG, LANG};

pub fn get_translation(key: &str) -> String {
    LANG.get(key).unwrap_or(DEFAULT_LANG.get(key).unwrap()).to_string()
}
