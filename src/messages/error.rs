use crate::app::types::App;

pub trait Error {
    fn error(&mut self, message: String);
}

impl Error for App {
    fn error(&mut self, message: String) {
        println!("Error: {message}")
    }
}
