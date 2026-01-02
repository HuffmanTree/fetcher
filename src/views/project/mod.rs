mod input_pane;
mod management_pane;
mod output_pane;

use crate::app::types::App;
use crate::messages::types::Message;
use crate::projects::types::Project;
use crate::views::project::input_pane::view as input_pane_view;
use crate::views::project::management_pane::view as management_pane_view;
use crate::views::project::output_pane::view as output_pane_view;
use iced::widget::{row, text};
use iced::{Element, Length};

fn project_view(project: &Project) -> Element<'_, Message> {
    row![
        management_pane_view(project).center(Length::FillPortion(1)),
        input_pane_view(project).center(Length::FillPortion(2)),
        output_pane_view(project).center(Length::FillPortion(2))
    ]
    .into()
}

pub fn view(app: &App, project_id: String) -> Element<'_, Message> {
    if let Some(project) = app.get_project(project_id.clone()) {
        project_view(project)
    } else {
        text(format!("Project not found {}", project_id)).into()
    }
}
