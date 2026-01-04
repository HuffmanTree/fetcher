use crate::app::types::App;

pub trait ExecuteRequest {
    fn execute_request(&self, project_id: String, request_id: String) -> Result<(), String>;
}

impl ExecuteRequest for App {
    fn execute_request(&self, project_id: String, request_id: String) -> Result<(), String> {
        if let Some(project) = self.get_project(project_id) {
            if let Some(request) = project.get_request(request_id) {
                request.execute()
            } else {
                Err(String::from("Request not found"))
            }
        } else {
            Err(String::from("Project not found"))
        }
    }
}
