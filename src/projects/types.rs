use crate::requests::types::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
    requests: Vec<Request>,
}
