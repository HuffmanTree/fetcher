use crate::messages::types::Message;
use crate::projects::types::Project;
use crate::views::types::View;
use chrono::DateTime;
use iced::font::Weight;
use iced::widget::{button, column, text};
use iced::{Element, Font};

fn timestamp_to_date_string(timestamp: i64) -> String {
    let date = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();

    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn component(project: &Project) -> Element<'_, Message> {
    let name = text(project.name.clone());
    let created_at = text(format!(
        "Created at: {}",
        timestamp_to_date_string(project.created_at)
    ));
    let updated_at = text(format!(
        "Updated at: {}",
        timestamp_to_date_string(project.updated_at)
    ));

    let col = column![
        name.font(Font {
            weight: Weight::Bold,
            ..Font::default()
        }),
        created_at.font(Font::MONOSPACE),
        updated_at.font(Font::MONOSPACE)
    ];

    button(col)
        .on_press(Message::GoTo(View::Project(project.id.clone())))
        .into()
}
