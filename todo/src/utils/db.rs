use crate::utils::todo::Todo;
use rusqlite::{Connection, Result};

pub struct Db {
    conn: Connection,
    db_path: String,
}

impl Db {
    pub fn new(db_path: String) -> Result<Db> {
        let conn = Connection::open(&db_path)?;
        Ok(Db { conn, db_path })
    }

    pub fn get_db_path(&self) -> &str {
        &self.db_path
    }

    pub fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    priority INTEGER NOT NULL
                )",
            [],
        )?;
        Ok(())
    }

    pub fn add_todo(&self, name: &str, description: &str, priority: i32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO todos (name, description, priority) VALUES (?1, ?2, ?3)",
            (&name, &description, &priority),
        )?;
        Ok(())
    }

    pub fn remove_todo(&self, id: i32) -> Result<()> {
        self.conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, priority FROM todos")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
            })
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }

        Ok(todos)
    }
}
