use std::rc::Rc;
use gtk4::{DropDown, Grid, Label};
use gtk4::prelude::GridExt;

pub type Components = (DropDown,Label);

pub fn get_components() -> Components {
    (
        DropDown::from_strings(&["Hello", "World"]),
        Label::builder()
            .label("0")
            .build()
    )
}

pub fn attach_components(grid: &Grid, components: &Rc<Components>) {
    grid.attach(&components.0, 0, 0, 1, 1);
    grid.attach(&components.1, 0, 1, 1, 1);
}