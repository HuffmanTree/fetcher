use crate::app::types::App;
use crate::config::get_data_home;
use std::fs;

pub trait LoadConfig {
    fn load_config(&mut self) -> Result<(), String>;
}

impl LoadConfig for App {
    fn load_config(&mut self) -> Result<(), String> {
        let data_home = get_data_home();
        fs::create_dir_all(data_home.clone()).map_err(|err| err.to_string())?;
        self.data_home = Some(data_home);

        Ok(())
    }
}
