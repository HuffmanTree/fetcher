use crate::messages::types::Message;
use crate::mutations::project::ProjectMutation;
use crate::projects::types::Project;
use crate::requests::types::Request;
use iced::widget::button::Style;
use iced::widget::{button, text};
use iced::{Background, Color, Element, Length};

pub fn component<'a>(project: &'a Project, request: &'a Request) -> Element<'a, Message> {
    let background = match project.current_request.clone() {
        Some(current) => {
            if current == request.id {
                Some(Background::Color(Color::BLACK))
            } else {
                None
            }
        }
        None => None,
    };
    button(text(request.name.as_str()))
        .width(Length::Fill)
        .style(move |theme, status| Style {
            background,
            ..button::primary(theme, status)
        })
        .on_press(Message::MutateProject(
            project.id.clone(),
            ProjectMutation::CurrentRequest(request.id.clone()),
        ))
        .into()
}
