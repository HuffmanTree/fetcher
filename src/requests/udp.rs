use crate::requests::types::Request;
use std::net::UdpSocket;

pub fn execute(request: &Request) -> Result<(), String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|err| err.to_string())?;
    socket
        .send_to(request.data.as_bytes(), request.url.as_str())
        .map_err(|err| err.to_string())?;

    Ok(())
}
