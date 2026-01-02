use crate::views::types::View;

#[derive(Clone, Debug)]
pub enum Message {
    Error(String),
    GoTo(View),
    ImportProjects,
    LoadConfig,
}
