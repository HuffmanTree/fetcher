pub mod types;

use crate::app::types::App;
use crate::debug;
use crate::messages::error::Error;
use crate::messages::execute_request::ExecuteRequest;
use crate::messages::goto::GoTo;
use crate::messages::import_projects::ImportProjects;
use crate::messages::load_config::LoadConfig;
use crate::messages::mutate::Mutate;
use crate::messages::types::Message;
use crate::projects::types::Project;
use crate::views::app::view;
use crate::views::types::View;
use iced::{Element, Task};

fn wrap(res: Result<(), String>) -> Task<Message> {
    match res {
        Ok(_) => Task::none(),
        Err(message) => Task::done(Message::Error(message)),
    }
}

impl App {
    pub fn get_project(&self, project_id: String) -> Option<&Project> {
        self.projects
            .iter()
            .find(|project| project.id == project_id)
    }

    pub fn get_project_mut(&mut self, project_id: String) -> Option<&mut Project> {
        self.projects
            .iter_mut()
            .find(|project| project.id == project_id)
    }

    pub fn boot() -> (Self, Task<Message>) {
        (
            App::default(),
            Task::done(Message::LoadConfig)
                .chain(Task::done(Message::ImportProjects))
                .chain(Task::done(Message::GoTo(View::Projects))),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        debug!("{:?}", message);

        match message {
            Message::GoTo(view) => {
                self.goto(view);
                Task::none()
            }
            Message::Error(message) => {
                self.error(message);
                Task::none()
            }
            Message::ImportProjects => wrap(self.import_projects()),
            Message::LoadConfig => wrap(self.load_config()),
            Message::MutateProject(project_id, mutation) => {
                wrap(self.mutate_project(project_id, mutation))
            }
            Message::MutateRequest(project_id, request_id, mutation) => {
                wrap(self.mutate_request(project_id, request_id, mutation))
            }
            Message::ExecuteRequest(project_id, request_id) => {
                wrap(self.execute_request(project_id, request_id))
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        view(self)
    }
}
