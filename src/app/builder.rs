use std::rc::Rc;
use gtk4::{Align, ApplicationWindow, Grid};
use gtk4::Application;
use gtk4::prelude::GtkWindowExt;
use crate::app::components::{attach_components, get_components};
use crate::app::listeners::subscribe;
use crate::utils::get_translation;

pub fn build_app(app: &Application) {
    let grid = build_main_grid();
    let window = build_main_window(app, &grid);
    let components = Rc::new(get_components());

    subscribe(&components);
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