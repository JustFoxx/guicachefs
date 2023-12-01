use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use gtk4::{Align, Application, ApplicationWindow, Button, glib, Text};
use gtk4::{Grid};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, ButtonExt, EditableExt, GridExt, WidgetExt};
use gtk4::traits::GtkWindowExt;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    println!("Test");
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let grid = &Grid::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .resizable(false)
        .title("Guicachefs")
        .child(grid)
        .build();

    let components = Rc::new((
        Button::builder()
            .label("Click me!")
            .build(),

        Text::builder()
            .editable(true)
            .text("cool")
            .build(),
    ));

    let i = AtomicU32::new(1);
    let copy_text = components.1.clone();

    components.0.connect_clicked(move |btn| {
        let i = i.fetch_add(1, Ordering::SeqCst);
        btn.set_label(&format!("{i} clicks!"));
        println!("{}", copy_text.text())
    });

    grid.attach(&components.0, 0, 0, 10, 1);
    grid.attach_next_to(&components.1, Some(&components.0), gtk4::PositionType::Right, 1, 1);

    window.present();
}
