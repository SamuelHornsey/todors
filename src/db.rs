use rusqlite::{ Connection, Error, Result, params };
use home::home_dir;

use crate::todo::Todo;


fn get_conn() -> Result<Connection, Error> {
    let home = home_dir().expect("unable to get home dir");
    let path = home.join(".todos/todos.db");
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub fn init_db() -> Result<()> {
    let conn = get_conn()?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            task TEXT NOT NULL,
            completed BOOL)", ())?;
    Ok(())
}

pub fn get_all() -> Result<Vec<Todo>> {
    let conn = get_conn()?;
    let mut stmt = conn.prepare("SELECT id, task, completed FROM todos")?;
    let iter = stmt.query_map([], |row| {
        Ok(Todo::marshal(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    let mut todos = Vec::new();
    for todo in iter {
        todos.push(todo?);
    }

    Ok(todos)
}

pub fn save(todo: Todo) -> Result<()> {
    let conn = get_conn().expect("unable to get connection");
    conn.execute(
        "INSERT INTO todos (task, completed) VALUES (?, ?)", (&todo.task, &todo.completed))?;
    Ok(())
}

pub fn delete_by_id(todo: Todo) -> Result<()> {
    let conn = get_conn()?;

    if let Some(id) = todo.id {
        conn.execute(
            "DELETE FROM todos WHERE id = ?", params![id])?;
        Ok(())
    } else {
        Err(rusqlite::Error::InvalidQuery)
    }
}

pub fn get_by_task(task: String) -> Result<Todo> {
    let conn = get_conn()?;

    let mut stmt = conn.prepare("SELECT * FROM todos WHERE task = ? LIMIT 1")?;
    let mut rows = stmt.query(params![task])?;

    if let Some(row) = rows.next()? {
        Ok(Todo {
            id: row.get(0)?,
            task: row.get(1)?,
            completed: row.get(2)?
        })
    } else {
        Err(rusqlite::Error::InvalidQuery)
    }
}

pub fn complete_todo_by_task(todo: Todo) -> Result<()> {
    let conn = get_conn()?;
    
    if let Some(id) = todo.id {
        conn.execute(
            "UPDATE todos SET completed = true WHERE id = ?", params![id])?;
        Ok(())
    } else {
        Err(rusqlite::Error::InvalidQuery)
    }
}
