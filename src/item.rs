use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Validate)]
pub struct TodoItem {
    #[validate(custom = "crate::validation::is_valid_author")]
    pub author: String,

    #[validate(custom = "crate::validation::is_valid_title")]
    pub title: String,

    pub description: String,
}
