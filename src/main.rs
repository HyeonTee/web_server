mod thread_pool;
mod handler;
mod http;
mod utils;

use dotenvy::dotenv;
use std::env;
use num_cpus;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use thread_pool::ThreadPool;
use http::{request::Request, response::Response, status::StatusCode};
use handler::route::route;

fn main() {
    dotenv().ok();
    let num_threads = env::var("THREAD_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(num_cpus::get);
    
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    
    let pool = ThreadPool::new(num_threads);

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
            let res = Response::new(StatusCode::BadRequest, "Bad Request", "text/plain");
            stream.write_all(res.to_string().as_bytes()).unwrap();
            return;
        }
    };

    let response = route(&request);
    stream.write_all(response.to_string().as_bytes()).unwrap();
}