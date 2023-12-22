use gtk4::glib::once_cell::sync::Lazy;
use include_dir::include_dir;

static LANG_DIR: include_dir::Dir = include_dir!("langs");

pub static LANG: Lazy<LangMap> = Lazy::new(generate_language);

pub static DEFAULT_LANG: Lazy<LangMap> = Lazy::new(generate_default_language);

const DEFAULT_LANG_LOCALE: &str = "en-us";

fn generate_default_language() -> LangMap {
    let file = LANG_DIR.get_file(format!("{DEFAULT_LANG_LOCALE}.json")).unwrap();
    let contents = file.contents_utf8().unwrap();
    serde_json::from_str(contents).unwrap()
}

fn generate_language() -> LangMap {
    let lang = sys_locale::get_locale().unwrap_or("en-US".to_string()).to_lowercase();
    let file = LANG_DIR.get_file(format!("{lang}.json"));
    if let Some(file) = file {
        let contents = file.contents_utf8().unwrap();
        serde_json::from_str(content).unwrap()
    } else {
        println!("Language {lang} is not yet supported");
        generate_default_language()
    }
}

#[derive(Debug, Deserialize)]
pub struct LangMap {
    #[serde(rename = "main-window.title")]
    pub main_window_title: String
}
