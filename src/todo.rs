use std::fmt;

#[derive(Debug)]
pub struct Todo {
    pub id: Option<i32>,
    pub task: String,
    pub completed: bool
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.completed {
            write!(f, "✅ {}", self.task)
        } else {
            write!(f, "❌ {}", self.task)
        }
    }
}

impl Todo {
    pub fn new(task: String, completed: bool) -> Self {
        Todo {
            id: None,
            task,
            completed
        }
    }

    pub fn marshal(id: i32, task: String, completed: bool) -> Self {
        Todo {
            id: Some(id), task, completed
        }
    }
}