use std::rc::Rc;
use gtk4::{Button, DropDown, Label};
use gtk4::prelude::WidgetExt;
use crate::app::components::Components;

pub fn subscribe(components: &Rc<Components>) {
    components.connect_selected_item_notify(on_dropdown_selected_item_0());
}

fn on_dropdown_selected_item_0() -> impl Fn(&DropDown) {
    move |drop_down: &DropDown| {
        let selected = drop_down.selected();
        drop_down.set_tooltip_text(Some(&selected.to_string()));
        println!("{}", selected);
    }
}