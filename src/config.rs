use std::env;

pub struct Config {
    pub bind_addr: String,
    pub port: String,
    pub thread_pool_size: usize,
}

impl Config {
    pub fn from_env() -> Self {
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let thread_pool_size = env::var("THREAD_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or_else(num_cpus::get);

        Self {
            bind_addr,
            port,
            thread_pool_size,
        }
    }

    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.bind_addr, self.port)
    }
}
