mod todo;

use clap::Parser;
// use todo::Todo;
use todo::Todo;

#[derive(Parser)]
struct Cli {
    command: String,
    key: String,
}

fn main() {

    let something = String::from("hello world");
    let todo = Todo {
        name: String::from("Buy groceries"),
        description: String::from("Buy milk, eggs, and bread"),
        priority: 1,
    };
    let args = Cli::parse();
    println!("Command line: {} {}", args.command, args.key);

    let command = args.command;
    match command.as_str() {
        "add" => {
            println!("Adding a new todo");
        },
        "remove" => {
            println!("Removing a todo");
        },
        "list" => {
            println!("Listing all todos");
        },
        _ => {
            println!("Unknown command");
        }
    }
}
