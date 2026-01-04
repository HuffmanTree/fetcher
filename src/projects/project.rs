use crate::projects::types::{Project, ShallowProject};
use crate::requests::types::Request;

impl Project {
    pub fn get_current_request(&self) -> Option<&Request> {
        self.current_request
            .clone()
            .and_then(|request_id| self.get_request(request_id))
    }

    pub fn get_request(&self, request_id: String) -> Option<&Request> {
        self.requests
            .iter()
            .find(|request| request.id == request_id)
    }

    pub fn get_request_mut(&mut self, request_id: String) -> Option<&mut Request> {
        self.requests
            .iter_mut()
            .find(|request| request.id == request_id)
    }

    pub fn set_current_request(&mut self, request_id: String) {
        self.current_request = Some(request_id)
    }
}

impl ShallowProject {
    pub fn deepen(self) -> Project {
        Project {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            requests: self.requests,
            current_request: None,
        }
    }
}
