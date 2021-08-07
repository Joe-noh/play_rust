use std::fmt;

pub struct TodoList {
    todos: Vec<Todo>,
}

struct Todo {
    done: bool,
    body: String,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    pub fn add(&mut self, body: &str) {
        self.todos.push(Todo {
            done: false,
            body: body.to_string(),
        })
    }

    pub fn remove(&mut self, indices: Vec<i32>) {
        let mut index = 0;

        self.todos.retain(|_| {
            index += 1;
            !indices.contains(&(index - 1))
        })
    }

    pub fn remove_all(&mut self) {
        self.todos.clear()
    }

    pub fn toggle(&mut self, indices: Vec<i32>) {
        let mut index = 0;

        for todo in &mut self.todos {
            if indices.contains(&index) {
                todo.done = !todo.done
            }

            index += 1;
        }
    }
}

fn checkbox(todo: &Todo) -> &str {
    if todo.done {
        "[x]"
    } else {
        "[ ]"
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let lines = self
            .todos
            .as_slice()
            .iter()
            .enumerate()
            .map(|(i, todo)| format!("{}: {} {}", i, checkbox(todo), todo.body))
            .collect::<Vec<_>>()
            .join("\n");

        formatter.write_str(&lines)
    }
}

#[test]
fn test_add() {
    let mut todo_list = TodoList::new();

    todo_list.add("Buy some more milk");
    assert_eq!(todo_list.todos.len(), 1);

    todo_list.add("Read the book");
    assert_eq!(todo_list.todos.len(), 2);
}

#[test]
fn test_remove() {
    let mut todo_list = TodoList::new();

    todo_list.add("Buy some more milk");
    todo_list.add("Read the book");

    todo_list.remove(vec![0]);

    assert_eq!(todo_list.todos[0].body, "Read the book");
}

#[test]
fn test_remove_all() {
    let mut todo_list = TodoList::new();

    todo_list.add("Buy some more milk");
    todo_list.add("Read the book");

    todo_list.remove_all();

    assert_eq!(todo_list.todos.len(), 0);
}

#[test]
fn test_toggle() {
    let mut todo_list = TodoList::new();

    todo_list.add("Buy some more milk");
    todo_list.add("Read the book");

    todo_list.toggle(vec![0]);
    assert!(todo_list.todos[0].done);

    todo_list.toggle(vec![0]);
    assert!(!todo_list.todos[0].done);
}

#[test]
fn test_display() {
    let mut todo_list = TodoList::new();

    todo_list.add("Buy some more milk");
    todo_list.add("Read the book");
    todo_list.toggle(vec![1]);

    assert_eq!(
        format!("{}", todo_list),
        "0: [ ] Buy some more milk\n1: [x] Read the book"
    );
}
