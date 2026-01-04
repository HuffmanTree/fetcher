use crate::mutations::project::ProjectMutation;
use crate::mutations::request::RequestMutation;
use crate::views::types::View;

#[derive(Clone, Debug)]
pub enum Message {
    Error(String),
    GoTo(View),
    ImportProjects,
    LoadConfig,
    ExecuteRequest(String, String),
    MutateProject(String, ProjectMutation),
    MutateRequest(String, String, RequestMutation),
}
