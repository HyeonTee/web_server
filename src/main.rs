mod thread_pool;
mod handler;
mod http;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use thread_pool::ThreadPool;
use http::{request::Request, response::Response};
use handler::route::route;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = match Request::from(&buffer) {
        Some(req) => req,
        None => {
            let res = Response::new("HTTP/1.1 400 Bad Request", "Bad Request", "text/plain");
            stream.write_all(res.to_string().as_bytes()).unwrap();
            return;
        }
    };

    let response = route(&request);
    stream.write_all(response.to_string().as_bytes()).unwrap();
}