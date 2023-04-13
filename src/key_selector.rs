use std::fs;
use std::path::Path;

static KEY_FOLDER: &str = "key";
static DEFAULT_KEY_NAME: &str = "default";

pub async fn select_default_key() -> String {
    if std::env::var("OPENAI_API_KEY").is_ok() {
        return std::env::var("OPENAI_API_KEY").unwrap();
    }

    let default_key_path = Path::new(KEY_FOLDER).join(DEFAULT_KEY_NAME);
    fs::read_to_string(default_key_path).unwrap()
}
