use crate::{db::{complete_todo_by_task, delete_by_id, get_all, get_by_task, save}, todo::Todo};

pub fn add(task: &String) {
    let todo = Todo::new(task.to_string(), false);
    let _ = save(todo);

    println!("saved: {}", task);
}

pub fn list() {
    let todos = get_all().expect("unable to get todos");

    for todo in todos {
        println!("{}", todo);
    } 
}

pub fn delete(task: &String) {
    let todo = get_by_task(task.to_string()).expect("unable to get todo");
    let _ = delete_by_id(todo);

    println!("deleted: {}", task);
}

pub fn complete(task: &String) {
    let todo = get_by_task(task.to_string()).expect("unable to get todo");
    let _ = complete_todo_by_task(todo);

    println!("completed: {}", task);
}