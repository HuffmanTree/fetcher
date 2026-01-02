use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    id: String,
    pub name: String,
}
