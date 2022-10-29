mod background;
mod config;
mod futil;
mod item;
mod routes;
mod validation;

use actix_cors::Cors;

use std::{io, thread};

use crate::config::Config;
use crate::item::TodoItem;
use actix_web::{web, App, HttpServer};
use linked_hash_set::LinkedHashSet;

fn load_config() -> io::Result<Config> {
    futil::read_or_init(
        "Server.toml",
        Config::default,
        |t| toml::to_string(t).unwrap(),
        |d| toml::from_str(d).expect("Malformed Server.toml file"),
    )
}

fn load_posts() -> io::Result<LinkedHashSet<TodoItem>> {
    futil::read_or_init(
        "posts.json",
        LinkedHashSet::new,
        |t| serde_json::to_string(t).unwrap(),
        |d| serde_json::from_str(d).expect("posts.json file is either malformed or outdated"),
    )
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Loading server settings from Server.toml...");

    let config = load_config()?;
    println!("Using configuration: {config:#?}");

    println!("Spawning thread for scheduled data backups...");
    thread::spawn(move || background::save_io(config.backup_interval));

    println!("Loading posts...");
    let posts = load_posts()?;

    routes::init_posts(posts).unwrap();

    println!("Starting server!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_origin(&config.request_origin);

        App::new()
            .wrap(cors)
            .route(
                "/post",
                web::post().to(move |p| routes::post(config.post_character_limit, p)),
            )
            .service(routes::view)
    })
    .bind(config.bind_addr)?
    .run()
    .await
}
