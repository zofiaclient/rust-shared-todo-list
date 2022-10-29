static TODO: Lazy<Mutex<LinkedHashSet<TodoItem>>> = Lazy::new(|| Mutex::new(LinkedHashSet::new()));

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize, Validate)]
pub struct TodoItem {
    #[validate(custom = "validation::is_valid_author")]
    author: String,

    #[validate(custom = "validation::is_valid_title")]
    title: String,

    description: String,
}

pub fn acquire_posts<'a>() -> MutexGuard<'a, LinkedHashSet<TodoItem>> {
    TODO.lock()
        .expect("Could not acquire lock on TODO because the Mutex was poisoned")
}