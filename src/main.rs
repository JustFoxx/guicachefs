mod frontend;
mod backend;

use std::collections::HashMap;
use gtk4::glib::ExitCode;
use crate::frontend::start_app;

const APP_ID: &str = "io.github.justfoxx.guicachefs";

type LangMap = HashMap<String,String>;

fn get_string() {

}

fn main() -> Result<(), String> {
    match start_app() {
        ExitCode::SUCCESS => Ok(()),
        ExitCode::FAILURE => Err("Failed to start application".to_string()),
        x => Err(format!("Unknown exit code: {}",x.value())),
    }
}

