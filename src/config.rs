pub use std::sync::Mutex;
pub use std::sync::OnceLock;

pub static MUTABLE_CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub index_txt_path: String,
}

pub fn initialize_mutable_config(new_index_txt_path: String) {
    MUTABLE_CONFIG
        .set(Mutex::new(Config {
            index_txt_path: new_index_txt_path,
        }))
        .expect("Mutable config already initialized");
}

pub fn update_index_txt_path(new_index_txt_path: String) {
    let config_lock = MUTABLE_CONFIG.get().expect("Config not initialized");
    let mut config = config_lock.lock().unwrap(); // Acquire lock
    config.index_txt_path = new_index_txt_path;
}
