use crate::messages::types::Message;
use crate::requests::types::Request;
use iced::widget::{Container, container, text};

pub fn view(request: &Request) -> Container<'_, Message> {
    container(text(format!("Output {}", request.name)))
}
