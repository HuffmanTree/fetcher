use crate::requests::types::Request;
use std::io::Write;
use std::net::TcpStream;

pub fn execute(request: &Request) -> Result<(), String> {
    let mut stream = TcpStream::connect(request.url.as_str()).map_err(|err| err.to_string())?;
    stream
        .write_all(request.data.as_bytes())
        .map_err(|err| err.to_string())?;

    Ok(())
}
