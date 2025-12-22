pub mod types;

use crate::app::types::App;
use crate::messages::types::Message;
use iced::Element;

impl App {
    pub fn boot() -> Self {
        App::default()
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        todo!()
    }
}
