use crate::app::types::App;
use crate::components::project_selector::component as project_selector_component;
use crate::messages::types::Message;
use iced::Element;
use iced::widget::{container, row};

pub fn view(app: &App) -> Element<'_, Message> {
    let projects = row(app.projects.iter().map(project_selector_component));

    container(projects.spacing(12)).into()
}
