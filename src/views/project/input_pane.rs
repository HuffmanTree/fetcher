use crate::messages::types::Message;
use crate::projects::types::Project;
use iced::widget::{Container, container, text};

pub fn view(project: &Project) -> Container<'_, Message> {
    container(text(format!("Input {}", project.name)))
}
