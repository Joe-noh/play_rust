use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Write};
use std::path::Path;

use super::TodoList;

fn load(path: &Path) -> Result<TodoList, Box<dyn Error>> {
    if Path::new(path).exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        match serde_json::from_reader(reader) {
            Ok(list) => Ok(list),
            Err(_e) => Ok(TodoList::new()),
        }
    } else {
        File::create(path)?;
        let list = TodoList::new();

        Ok(list)
    }
}

fn save(path: &Path, todo_list: TodoList) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;
    let json = serde_json::to_string(&todo_list)?;

    match write!(file, "{}", json) {
        Ok(_r) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

#[test]
fn test_load_file_missing() {
    use tempfile::NamedTempFile;

    let file = NamedTempFile::new().unwrap();
    let list = load(file.path()).unwrap();

    assert_eq!(list.todos.len(), 0);
}

#[test]
fn test_load_file_exists() {
    use tempfile::NamedTempFile;

    let file = NamedTempFile::new().unwrap();
    let mut todo_list = TodoList::new();

    todo_list.add("Buy some more milk");

    save(file.path(), todo_list)
        .and_then(|_r| {
            let loaded = load(file.path()).unwrap();

            assert_eq!(loaded.todos.len(), 1);
            assert_eq!(loaded.todos.first().unwrap().body, "Buy some more milk");

            Ok(())
        })
        .unwrap();
}
