use crate::mutations::Mutation;
use crate::projects::types::Project;

#[derive(Clone, Debug)]
pub enum ProjectMutation {
    CurrentRequest(String),
}

impl Mutation for ProjectMutation {
    type Item = Project;

    fn apply(self, item: &mut Self::Item) {
        match self {
            ProjectMutation::CurrentRequest(request_id) => item.set_current_request(request_id),
        }
    }
}
