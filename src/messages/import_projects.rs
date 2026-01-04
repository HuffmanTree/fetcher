use crate::app::types::App;
use crate::projects::types::ShallowProject;
use std::fs;

pub trait ImportProjects {
    fn import_projects(&mut self) -> Result<(), String>;
}

impl ImportProjects for App {
    fn import_projects(&mut self) -> Result<(), String> {
        match self.data_home.clone() {
            Some(data_home) => {
                let projects_file = data_home.join("projects.json");
                let project_json =
                    fs::read_to_string(projects_file).map_err(|err| err.to_string())?;
                let projects = serde_json::from_str::<Vec<ShallowProject>>(project_json.as_str())
                    .map_err(|err| err.to_string())?;
                self.projects = projects
                    .into_iter()
                    .map(|project| project.deepen())
                    .collect();

                Ok(())
            }
            _ => Err(String::from("Config not loaded")),
        }
    }
}
