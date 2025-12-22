pub mod types;

use crate::app::types::App;
use crate::debug;
use crate::messages::goto::GoTo;
use crate::messages::types::Message;
use iced::Element;

impl App {
    pub fn boot() -> Self {
        App::default()
    }

    pub fn update(&mut self, message: Message) {
        debug!("{:?}", message);

        match message {
            Message::GoTo(view) => self.goto(view),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        todo!()
    }
}
