use std::net::TcpStream;
use std::io::{Read, Write};

use crate::http::{Request, StatusCode};
use crate::router::Router;
use crate::error::json_error_response;

const READ_BUFFER_SIZE: usize = 16 * 1024;

pub fn handle(mut stream: TcpStream, router: &Router) {
    let mut buffer = [0u8; READ_BUFFER_SIZE];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(n) => n,
        Err(_) => return,
    };

    let request = match Request::from(&buffer[..bytes_read]) {
        Some(req) => req,
        None => {
            let res = json_error_response(StatusCode::BadRequest, "Malformed HTTP request");
            let _ = stream.write_all(&res.to_bytes());
            return;
        }
    };

    let response = router.handle(&request);
    let _ = stream.write_all(&response.to_bytes());
}
