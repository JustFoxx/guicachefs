use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use gtk4::{Align, Application, ApplicationWindow, Button, Grid, Text};
use gtk4::glib::ExitCode;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, ButtonExt, EditableExt, GridExt, GtkWindowExt, WidgetExt};
use crate::APP_ID;

pub fn start_app() -> ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_app);
    app.run()
}

fn build_app(app: &Application) {
    let grid = build_grid();
    let window = build_window(app, &grid);
    let components = Rc::new(build_components());
    build_callback(&components);
    attach_components(&grid, &components);
    window.present();
}

fn build_grid() -> Grid {
    Grid::builder()
        .halign(Align::Center)
        .column_spacing(10)
        .row_spacing(10)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build()
}

fn build_window(app: &Application, grid: &Grid) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .resizable(false)
        .title("Guicachefs")
        .child(grid)
        .build()
}

fn build_components() -> (Button, Text, Text, Text, Text) {
    (
        Button::builder()
            .label("Click me!")
            .build(),

        Text::builder()
            .text("cool")
            .build(),

        Text::builder()
            .editable(false)
            .build(),
        Text::builder()
            .editable(false)
            .build(),
        Text::builder()
            .editable(false)
            .build(),
    )
}

fn attach_components(grid: &Grid, components: &Rc<(Button, Text, Text, Text, Text)>) {
    grid.attach(&components.0, 0, 0, 1, 1);
    grid.attach(&components.1, 1, 0, 1, 1);
    grid.attach(&components.2, 0, 1, 1, 1);
    grid.attach(&components.3, 1, 1, 1, 1);
    grid.attach(&components.4, 2, 1, 1, 1);
}

fn build_callback(components: &Rc<(Button, Text, Text, Text, Text)>) {
    static I: AtomicU32 = AtomicU32::new(1);
    let string_text = components.1.clone();
    let (text0, text1, text2) = (components.2.clone(), components.3.clone(), components.4.clone());

    components.0.connect_clicked(move |btn| {
        let i = I.fetch_add(1, Ordering::SeqCst);
        btn.set_label(&format!("{i} clicks!"));
        println!("{}", string_text.text());
        if i % 2 == 0 {
            text0.set_text(&i.to_string());
            text1.set_text("");
            text2.set_text("");
        } else if i % 3 == 0 {
            text1.set_text(&i.to_string());
            text0.set_text("");
            text2.set_text("");
        } else {
            text2.set_text(&i.to_string());
            text0.set_text("");
            text1.set_text("");
        }
    });
}