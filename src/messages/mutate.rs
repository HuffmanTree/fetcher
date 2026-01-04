use crate::app::types::App;
use crate::mutations::Mutation;
use crate::mutations::project::ProjectMutation;
use crate::mutations::request::RequestMutation;

pub trait Mutate {
    fn mutate_project(
        &mut self,
        project_id: String,
        mutation: ProjectMutation,
    ) -> Result<(), String>;

    fn mutate_request(
        &mut self,
        project_id: String,
        request_id: String,
        mutation: RequestMutation,
    ) -> Result<(), String>;
}

impl Mutate for App {
    fn mutate_project(
        &mut self,
        project_id: String,
        mutation: ProjectMutation,
    ) -> Result<(), String> {
        if let Some(project) = self.get_project_mut(project_id) {
            Ok(mutation.apply(project))
        } else {
            Err(String::from("Project not found"))
        }
    }

    fn mutate_request(
        &mut self,
        project_id: String,
        request_id: String,
        mutation: RequestMutation,
    ) -> Result<(), String> {
        if let Some(project) = self.get_project_mut(project_id) {
            if let Some(request) = project.get_request_mut(request_id) {
                Ok(mutation.apply(request))
            } else {
                Err(String::from("Request not found"))
            }
        } else {
            Err(String::from("Project not found"))
        }
    }
}
