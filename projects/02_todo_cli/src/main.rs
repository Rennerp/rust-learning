use std::env;
use std::fs;
use std::io;
use std::path::Path;

const TODO_FILE: &str = "todos.txt";

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    // The first argument after the program name chooses the command.
    match args.get(1).map(String::as_str) {
        Some("add") => {
            let text = args.get(2..).unwrap_or(&[]).join(" ");
            if text.trim().is_empty() {
                return Err(String::from("usage: todo_cli add <text>"));
            }

            let mut todos = load_todos().map_err(|error| error.to_string())?;
            todos.push(text);
            save_todos(&todos).map_err(|error| error.to_string())?;
            println!("added");
            Ok(())
        }
        Some("list") => {
            let todos = load_todos().map_err(|error| error.to_string())?;
            for (index, todo) in todos.iter().enumerate() {
                println!("{}: {}", index + 1, todo);
            }
            Ok(())
        }
        Some("done") => {
            let index = args
                .get(2)
                .ok_or_else(|| String::from("usage: todo_cli done <number>"))?
                .parse::<usize>()
                .map_err(|_| String::from("todo number must be valid"))?;

            let mut todos = load_todos().map_err(|error| error.to_string())?;
            if index == 0 || index > todos.len() {
                return Err(String::from("todo number is out of range"));
            }

            todos.remove(index - 1);
            save_todos(&todos).map_err(|error| error.to_string())?;
            println!("done");
            Ok(())
        }
        _ => {
            println!("usage:");
            println!("  todo_cli add <text>");
            println!("  todo_cli list");
            println!("  todo_cli done <number>");
            Ok(())
        }
    }
}

fn load_todos() -> io::Result<Vec<String>> {
    if !Path::new(TODO_FILE).exists() {
        return Ok(Vec::new());
    }

    // Each line in the file is one todo item.
    let contents = fs::read_to_string(TODO_FILE)?;
    Ok(contents.lines().map(String::from).collect())
}

fn save_todos(todos: &[String]) -> io::Result<()> {
    fs::write(TODO_FILE, todos.join("\n"))
}
