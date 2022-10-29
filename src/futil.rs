use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

pub fn read_or_init<'a, T, P, F, S, D>(
    path: P,
    initializer: F,
    serializer: S,
    deserializer: D,
) -> io::Result<T>
where
    T: Serialize + Deserialize<'a>,
    P: AsRef<Path>,
    F: FnOnce() -> T,
    S: FnOnce(&T) -> String,
    D: FnOnce(&str) -> T,
{
    let content = fs::read_to_string(&path);

    if let Err(err) = &content {
        if err.kind() == ErrorKind::NotFound {
            let t = initializer();
            fs::write(path, serializer(&t))?;
            return Ok(t);
        }
    }
    content.map(|s| deserializer(&s))
}
