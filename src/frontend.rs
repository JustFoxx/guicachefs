use std::rc::Rc;
use gtk4::{Align, Application, ApplicationWindow, DropDown, Grid, Label};
use gtk4::glib::{ExitCode};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GridExt, GtkWindowExt};
use crate::{APP_ID, get_translation, LANG};


pub fn start_app() -> ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_app);
    app.run()
}

fn build_app(app: &Application) {
    let grid = build_main_grid();
    let window = build_main_window(app, &grid);
    let components = Rc::new(build_components());
    build_callback(&components);
    attach_components(&grid, &components);
    window.present();
}

fn build_main_grid() -> Grid {
    Grid::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .column_spacing(10)
        .row_spacing(10)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build()
}

fn build_main_window(app: &Application, grid: &Grid) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .resizable(true)
        .title(get_translation("main-window.title"))
        .child(grid)
        .build()
}

fn build_components() -> (DropDown,Label) {
    (
        DropDown::from_strings(&["Hello", "World"]),
        Label::builder()
            .label("0")
            .build()
    )
}

fn attach_components(grid: &Grid, components: &Rc<(DropDown,Label)>) {
    grid.attach(&components.0, 0, 0, 1, 1);
    grid.attach(&components.1, 0, 1, 1, 1);
}

fn build_callback(components: &Rc<(DropDown,Label)>) {
    let text_component = components.1.clone();
    components.0.connect_selected_item_notify(move |drop_down| {
        let selected = drop_down.selected();
        text_component.set_text(&selected.to_string());
    });
}