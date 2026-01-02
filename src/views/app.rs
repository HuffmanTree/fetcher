use crate::app::types::App;
use crate::components::viewer::component as viewer_component;
use crate::messages::types::Message;
use iced::widget::container;
use iced::{Element, Length};

pub fn view(app: &App) -> Element<'_, Message> {
    container(viewer_component(app))
        .padding(12)
        .center(Length::Fill)
        .into()
}
