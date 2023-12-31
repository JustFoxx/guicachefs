mod components;
mod builder;
mod listeners;

use gtk4::Application;
use gtk4::glib::ExitCode;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use crate::app::builder::build_app;

pub fn start_app(app_id: &str) -> ExitCode {
    let app = Application::builder()
        .application_id(app_id)
        .build();

    app.connect_activate(build_app);
    app.run()
}