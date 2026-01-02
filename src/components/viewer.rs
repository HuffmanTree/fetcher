use crate::app::types::App;
use crate::messages::types::Message;
use crate::views::project::view as project_view;
use crate::views::projects::view as projects_view;
use crate::views::types::View;
use iced::Element;
use iced::widget::text;

pub fn component(app: &App) -> Element<'_, Message> {
    match app.current_view.clone() {
        Some(View::Project(project_id)) => project_view(app, project_id.clone()),
        Some(View::Projects) => projects_view(app),
        None => text("No view loaded").into(),
    }
}
