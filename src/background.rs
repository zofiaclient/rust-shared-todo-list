use crate::routes;
use std::time::Duration;
use std::{fs, thread};

pub fn save_io(backup_interval: u64) {
    loop {
        thread::sleep(Duration::from_secs(backup_interval));

        let json_str = serde_json::to_string(&*routes::acquire_posts()).unwrap();

        if let Err(err) = fs::write("posts.json", json_str) {
            eprintln!("Could not backup data: {err}");
        }
    }
}
