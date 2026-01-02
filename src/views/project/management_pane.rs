use crate::messages::types::Message;
use crate::projects::types::Project;
use crate::views::types::View;
use iced::Length;
use iced::widget::{Container, button, column, container, row, text};

fn head() -> Container<'static, Message> {
    let home_button = button(text("H")).on_press(Message::GoTo(View::Projects));

    container(row![home_button]).center(Length::Fill)
}

pub fn view(project: &Project) -> Container<'_, Message> {
    let head = head();
    let body = text(format!("Management {}", project.name));

    container(column![head.align_top(Length::Fill), body])
}
