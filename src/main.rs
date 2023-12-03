mod frontend;
mod backend;

use std::collections::HashMap;
use gtk4::glib::ExitCode;
use gtk4::glib::once_cell::sync::Lazy;
use include_dir::{include_dir};
use crate::frontend::start_app;

const APP_ID: &str = "io.github.justfoxx.guicachefs";

static LANG_DIR: include_dir::Dir = include_dir!("langs");

type LangMap = HashMap<String,String>;
static LANG: Lazy<LangMap> = Lazy::new(|| {
    let lang = sys_locale::get_locale().unwrap_or("en-US".to_string()).to_lowercase();
    let file = LANG_DIR.get_file(format!("{lang}.json")).unwrap_or_else(|| {
        println!("Language {lang} is yet not supported");
        LANG_DIR.get_file("en-us.json").unwrap()
    });
    let contents = file.contents_utf8().unwrap();
    serde_json::from_str(contents).unwrap()
});

static DEFAULT_LANG: Lazy<LangMap> = Lazy::new(|| {
    let file = LANG_DIR.get_file("en-us.json").unwrap();
    let contents = file.contents_utf8().unwrap();
    serde_json::from_str(contents).unwrap()
});

fn get_translation(key: &str) -> String {
    LANG.get(key).unwrap_or(DEFAULT_LANG.get(key).unwrap()).to_string()
}

fn main() -> Result<(), String> {
    match start_app() {
        ExitCode::SUCCESS => Ok(()),
        ExitCode::FAILURE => Err("Failed to start application".to_string()),
        x => Err(format!("Unknown exit code: {}",x.value())),
    }
}

