use validator::ValidationError;

pub fn is_valid_author(author: &str) -> Result<(), ValidationError> {
    if !author.is_empty()
        && author.len() < 32
        && author.chars().all(|c| c.is_alphanumeric() || c == '_')
    {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid author"))
    }
}

pub fn is_valid_title(title: &str) -> Result<(), ValidationError> {
    if !title.is_empty() && title.len() < 64 && title.is_ascii() {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid title"))
    }
}
