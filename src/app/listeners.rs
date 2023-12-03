use std::rc::Rc;
use gtk4::{DropDown, Label};
use crate::app::components::Components;

pub fn subscribe(components: &Rc<Components>) {
    components.0.connect_selected_item_notify(on_dropdown_selected_item_0(components.1.clone()));
}

fn on_dropdown_selected_item_0(text_component: Label) -> impl Fn(&DropDown) {
    move |drop_down: &DropDown| {
        let selected = drop_down.selected();
        text_component.set_text(&selected.to_string());
        println!("{}", selected);
    }
}