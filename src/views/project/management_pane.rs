use crate::components::request_selector::component as request_selector_component;
use crate::messages::types::Message;
use crate::projects::types::Project;
use crate::views::types::View;
use iced::Length;
use iced::widget::{Container, button, column, container, row, text};

fn head() -> Container<'static, Message> {
    let home_button = button(text("H")).on_press(Message::GoTo(View::Projects));

    container(row![home_button]).center(Length::Fill)
}

fn body(project: &Project) -> Container<'_, Message> {
    let requests = column(
        project
            .requests
            .iter()
            .map(|request| request_selector_component(project, request)),
    );

    container(requests.spacing(12))
}

pub fn view(project: &Project) -> Container<'_, Message> {
    let head = head();
    let body = body(project);

    container(column![head.align_top(64), body.align_top(Length::Fill)])
}
