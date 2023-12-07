mod components;
mod builder;
mod listeners;

use libadwaita::{Application};
use gtk4::glib::ExitCode;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GridExt, GtkWindowExt};
use crate::app::builder::build_app;

pub fn start_app(app_id: &str) -> ExitCode {
    let app = Application::builder()
        .application_id(app_id)
        .build();

    app.connect_activate(build_app);
    app.run()
}