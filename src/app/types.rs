use crate::projects::types::Project;
use crate::views::types::View;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct App {
    pub current_view: Option<View>,
    pub data_home: Option<PathBuf>,
    pub projects: Vec<Project>,
}
