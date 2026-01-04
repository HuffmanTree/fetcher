mod input_pane;
mod management_pane;
mod output_pane;

use crate::app::types::App;
use crate::messages::types::Message;
use crate::projects::types::Project;
use crate::views::project::input_pane::view as input_pane_view;
use crate::views::project::management_pane::view as management_pane_view;
use crate::views::project::output_pane::view as output_pane_view;
use iced::widget::{container, row, text};
use iced::{Element, Length};

fn project_view(project: &Project) -> Element<'_, Message> {
    let request = project.get_current_request();
    let management_pane = management_pane_view(project);
    let input_pane = request.map_or(container(text("No request selected")), |request| {
        input_pane_view(project.id.clone(), request)
    });
    let output_pane = request.map_or(container(text("No request selected")), output_pane_view);

    row![
        management_pane.center(Length::FillPortion(1)),
        input_pane.center(Length::FillPortion(2)),
        output_pane.center(Length::FillPortion(2))
    ]
    .spacing(12)
    .into()
}

pub fn view(app: &App, project_id: String) -> Element<'_, Message> {
    if let Some(project) = app.get_project(project_id.clone()) {
        project_view(project)
    } else {
        text(format!("Project not found {}", project_id)).into()
    }
}
