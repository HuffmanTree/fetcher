use crate::projects::types::Project;
use crate::views::types::View;
use iced::widget::text_editor;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct App {
    pub current_view: Option<View>,
    pub data_home: Option<PathBuf>,
    pub projects: Vec<Project>,
    pub editors: HashMap<String, text_editor::Content>,
}
