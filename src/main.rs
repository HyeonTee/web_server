use dotenvy::dotenv;
use std::sync::Arc;
use std::net::TcpListener;

use web_server::config::Config;
use web_server::server::{connection, thread_pool::ThreadPool};
use web_server::router::Router;
use web_server::handlers::static_files;
use web_server::middleware::Logger;

fn main() {
    dotenv().ok();
    let config = Config::from_env();

    let router = Arc::new(
        Router::new()
            .wrap(Logger)
            .get("/", static_files::index)
            .get("/about", static_files::about)
            .get("/static/*", static_files::assets),
    );

    let listener = TcpListener::bind(config.listen_addr()).unwrap();
    let pool = ThreadPool::new(config.thread_pool_size);

    println!("Listening on {}", config.listen_addr());

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Accept error: {}", e);
                continue;
            }
        };
        let router = Arc::clone(&router);
        pool.execute(move || {
            connection::handle(stream, &router);
        });
    }
}
