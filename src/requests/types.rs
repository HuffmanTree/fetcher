use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: String,
    pub name: String,
    pub r#type: RequestType,
    pub url: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub enum RequestType {
    Tcp,
    Udp,
}
