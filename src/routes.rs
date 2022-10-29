use crate::item::TodoItem;
use actix_web::get;
use actix_web::web::Json;
use actix_web_validator::Query;
use linked_hash_set::LinkedHashSet;
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};

static TODO: OnceCell<Mutex<LinkedHashSet<TodoItem>>> = OnceCell::new();

pub fn acquire_posts<'a>() -> MutexGuard<'a, LinkedHashSet<TodoItem>> {
    TODO.get()
        .unwrap()
        .lock()
        .expect("Could not acquire lock on TODO because the Mutex was poisoned")
}

pub fn init_posts(posts: LinkedHashSet<TodoItem>) -> Result<(), Mutex<LinkedHashSet<TodoItem>>> {
    TODO.set(Mutex::new(posts))
}

pub async fn post(character_limit: usize, item: Query<TodoItem>) -> &'static str {
    let item = item.into_inner();

    if item.description.len() > character_limit {
        return "Character limit";
    }
    match acquire_posts().insert_if_absent(item) {
        true => "Ok",
        false => "Already exists",
    }
}

#[get("/view")]
pub async fn view() -> Json<Vec<TodoItem>> {
    Json(acquire_posts().clone().into_iter().collect())
}
