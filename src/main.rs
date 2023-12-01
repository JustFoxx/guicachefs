use std::sync::atomic::{AtomicU32, Ordering};
use gtk4::{Application, ApplicationWindow, Button, glib};
use gtk4::glib::PropertyGet;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, ButtonExt};
use gtk4::traits::GtkWindowExt;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    println!("Test");
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(app_main);
    app.run()
}

fn app_main(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .resizable(false)
        .title("Guicachefs")
        .build();

    let button = Button::builder()
        .label("Click me!")
        .margin_bottom(50)
        .margin_top(50)
        .margin_start(100)
        .margin_end(100)
        .build();
    static I: AtomicU32 = AtomicU32::new(1);
    button.connect_clicked(|btn| {
        let i = I.fetch_add(1, Ordering::SeqCst);
        btn.set_label(&format!("{i} clicks!"));
    });
    window.set_child(Some(&button));
    // Show the window.
    window.present();
}
