use std::rc::Rc;
use gtk4::{DropDown, Grid, StringList};
use gtk4::prelude::GridExt;

pub type Components = (DropDown);

pub static OPTIONS: &[&str; 21] = &[
    "",
    "format",
    "show-super",
    "set-option",
    "mount",
    "fsck",
    "fs usage",
    "device",
    "subvolume",
    "data",
    "unlock",
    "set-passphrase",
    "remove-passphrase",
    "migrate",
    "migrate-superblock",
    "setattr",
    "dump",
    "list",
    "list_journal",
    "fusemount",
    "version"
];

pub static DEVICES: &[&str; 9] = &[
    "",
    "add",
    "remove",
    "online",
    "offline",
    "evacuate",
    "set-state",
    "resize",
    "resize-journal"
];

pub static SUBVOLUMES: &[&str; 4] = &[
    "",
    "create",
    "delete",
    "snapshot"
];

pub static DATAS: &[&str; 2] = &[
    "replicate",
    "job"
];

pub fn get_components() -> Components {
    (
        DropDown::builder()
            .tooltip_text("Choose option")
            .model(&StringList::new(OPTIONS))
            .tooltip_text("Choose option. If the option is chosen, here will be description of it")
            .build()
    )
}

pub fn attach_components(grid: &Grid, components: &Rc<Components>) {
    grid.attach(components.as_ref(), 0, 0, 1, 1);
}