use crate::messages::types::Message;
use iced::Element;
use iced::widget::{button, text};

pub fn component(project_id: String, request_id: String) -> Element<'static, Message> {
    button(text("Send"))
        .on_press(Message::ExecuteRequest(project_id, request_id))
        .into()
}
