use crate::requests::types::{Request, RequestType};
use crate::requests::{tcp, udp};

impl Request {
    pub fn execute(&self) -> Result<(), String> {
        match self.r#type {
            RequestType::Tcp => tcp::execute(self),
            RequestType::Udp => udp::execute(self),
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url
    }
}
