mod components;
mod builder;
mod listeners;

use gtk4::{Application};
use gtk4::glib::ExitCode;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GridExt, GtkWindowExt};
use crate::app::builder::build_app;
use crate::APP_ID;

pub fn start_app() -> ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_app);
    app.run()
}