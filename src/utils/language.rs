use std::collections::HashMap;
use gtk4::glib::once_cell::sync::Lazy;
use include_dir::include_dir;

static LANG_DIR: include_dir::Dir = include_dir!("langs");

type LangMap = HashMap<String,String>;

pub static LANG: Lazy<LangMap> = Lazy::new(|| {
    let lang = sys_locale::get_locale().unwrap_or("en-US".to_string()).to_lowercase();
    let file = LANG_DIR.get_file(format!("{lang}.json")).unwrap_or_else(|| {
        println!("Language {lang} is yet not supported");
        LANG_DIR.get_file("en-us.json").unwrap()
    });
    let contents = file.contents_utf8().unwrap();
    serde_json::from_str(contents).unwrap()
});

pub static DEFAULT_LANG: Lazy<LangMap> = Lazy::new(|| {
    let file = LANG_DIR.get_file("en-us.json").unwrap();
    let contents = file.contents_utf8().unwrap();
    serde_json::from_str(contents).unwrap()
});