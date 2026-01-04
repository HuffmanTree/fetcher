use crate::messages::types::Message;
use crate::mutations::request::RequestMutation;
use crate::requests::types::Request;
use iced::widget::{Container, container, text, text_input};

pub fn head(project_id: String, request: &Request) -> Container<'_, Message> {
    let bar = text_input("TCP server address", request.url.as_str()).on_input(move |url| {
        Message::MutateRequest(
            project_id.clone(),
            request.id.clone(),
            RequestMutation::Url(url),
        )
    });

    container(bar)
}

pub fn body(request: &Request) -> Container<'_, Message> {
    container(text("This is TCP body"))
}
