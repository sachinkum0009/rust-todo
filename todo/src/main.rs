mod db;
mod todo;
mod config;

use clap::Parser;
use db::db::Db;
use std::io::{self, Write};
use prettytable::{Table, row};
use config::Config;

#[derive(Parser)]
struct Cli {
    command: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("Command line: {}", args.command);

    let mut task_name = String::new();
    let mut description = String::new();
    let mut priority = String::new();
    let mut index = String::new();

    let db_config = Config::new();
    let db_conn = Db::new(db_config.db_path).unwrap();
    db_conn.create_table()?;

    let command = args.command;
    println!("Welcome to Rust Todo App");
    match command.as_str() {
        "add" => {
            println!("Adding a new todo");

            print!("Enter task name: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut task_name)?;

            print!("Enter description: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut description)?;

            print!("Enter priority: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut priority)?;

            let priority: i32 = priority.trim().parse()?;
            println!("Task name: {}, Description: {}, Priority: {}", task_name.trim(), description.trim(), priority);

            db_conn.add_todo(&task_name.trim(), &description.trim(), priority)?;
            println!("Todo added successfully");
        },
        "remove" => {
            println!("Removing a todo");
            print!("Enter the index of the todo to remove: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut index)?;

            let index_: i32 = index.trim().parse()?;
            db_conn.remove_todo(index_)?;

        },
        "list" => {
            println!("Listing all todos");
            let todos = db_conn.get_todos()?;
            println!("List of Todos");
            let mut table = Table::new();
            table.add_row(row!["ID", "Task Name", "Description", "Priority"]);

            for todo in todos {
                table.add_row(row![todo.id, todo.name, todo.description, todo.priority]);
            }
            table.printstd();
        },
        "help" => {
            println!("Help");
        },
        _ => {
            println!("Unknown command");
        }
    }

    Ok(())
}
