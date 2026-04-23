pub use std::path::Path;
pub use std::path::PathBuf;
pub use std::sync::Mutex;
pub use std::sync::OnceLock;

pub static MUTABLE_CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub pdf_images: PathBuf,
    pub index_txt: PathBuf,
    pub bookmarks_txt: PathBuf,
}

pub fn initialize_mutable_config(dir: String) {
    let expanded_dir = shellexpand::tilde(&dir);
    let pdf_images = PathBuf::from(expanded_dir.into_owned());
    let index_txt = pdf_images.clone().join("index.txt");
    let bookmarks_txt = pdf_images.clone().join("bookmarks.txt");

    MUTABLE_CONFIG
        .set(Mutex::new(Config {
            pdf_images,
            index_txt,
            bookmarks_txt,
        }))
        .expect("Mutable config already initialized");
}

pub fn update_index_txt_path(dir: String) {
    let expanded_dir = shellexpand::tilde(&dir);
    let pdf_images = PathBuf::from(expanded_dir.into_owned());
    let index_txt = pdf_images.clone().join("index.txt");
    let bookmarks_txt = pdf_images.clone().join("bookmarks.txt");
    let config_lock = MUTABLE_CONFIG.get().expect("Config not initialized");
    let mut config = config_lock.lock().unwrap(); // Acquire lock
    config.pdf_images = pdf_images;
    config.index_txt = index_txt;
    config.bookmarks_txt = bookmarks_txt;
}
