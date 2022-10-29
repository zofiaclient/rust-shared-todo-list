use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub request_origin: String,
    pub bind_addr: SocketAddr,
    pub backup_interval: u64,
    pub post_character_limit: usize,
}

impl Config {
    const fn new(
        request_origin: String,
        bind_addr: SocketAddr,
        backup_interval: u64,
        post_character_limit: usize,
    ) -> Self {
        Self {
            request_origin,
            bind_addr,
            backup_interval,
            post_character_limit,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(
            // Default VSCode live preview address.
            "http://127.0.0.1:5500".to_string(),
            "127.0.0.1:8080".parse().unwrap(),
            30,
            2400,
        )
    }
}
