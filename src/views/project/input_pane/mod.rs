mod tcp_request;
mod udp_request;

use crate::components::request_executor::component as request_executor_component;
use crate::messages::types::Message;
use crate::requests::types::Request;
use crate::requests::types::RequestType;
use crate::views::project::input_pane::tcp_request::body as tcp_request_body;
use crate::views::project::input_pane::tcp_request::head as tcp_request_head;
use crate::views::project::input_pane::udp_request::body as udp_request_body;
use crate::views::project::input_pane::udp_request::head as udp_request_head;
use iced::Length;
use iced::widget::row;
use iced::widget::{Container, column, container};

pub fn view(project_id: String, request: &Request) -> Container<'_, Message> {
    let executor = request_executor_component(project_id.clone(), request.id.clone());
    let head = match &request.r#type {
        RequestType::Tcp => tcp_request_head(project_id, request),
        RequestType::Udp => udp_request_head(project_id, request),
    };
    let head = container(row![head, executor].spacing(12));
    let body = match &request.r#type {
        RequestType::Tcp => tcp_request_body(request),
        RequestType::Udp => udp_request_body(request),
    };

    container(column![head.align_top(64), body.align_top(Length::Fill)])
}
