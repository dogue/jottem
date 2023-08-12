fn default_root() -> String {
    if let Some(path) = dirs::data_dir() {
        path.join("jottem").to_string_lossy().into_owned()
    } else {
        eprintln!("Could not determine user data directory.");
        std::process::exit(1);
    }
}

fn default_db() -> String {
    if let Some(path) = dirs::cache_dir() {
        path.join("jottem").to_string_lossy().into_owned()
    } else {
        eprintln!("Could not determine user cache directory.");
        std::process::exit(2);
    }
}

pub fn get_root() -> String {
    std::env::var("JOTTEM_ROOT").unwrap_or_else(|_| default_root())
}

pub fn get_db_path() -> String {
    std::env::var("JOTTEM_DB_PATH").unwrap_or_else(|_| default_db())
}

pub fn get_editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string())
}
