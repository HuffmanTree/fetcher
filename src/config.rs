use std::env;
use std::path::PathBuf;

pub fn get_data_home() -> PathBuf {
    xdir::data()
        .unwrap_or_else(env::temp_dir)
        .join(env!("CARGO_PKG_NAME"))
}
