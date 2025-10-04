use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

// This derive macro allows our struct to be serialized to/from JSON.
// Clone is useful for creating copies, and Debug for printing.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: u32,
    task: String,
    completed: bool,
}

// Function to load todos from a JSON file
// If the file doesn't exist, it returns an empty list.
fn load_todos<P: AsRef<Path>>(path: P) -> Result<Vec<Todo>> {
    // Attempt to open the file in read-only mode.
    let file_result = File::open(path);

    match file_result {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .context("Failed to read from todo file")?;

            // If the file is empty, return an empty Vec
            if contents.is_empty() {
                return Ok(Vec::new());
            }

            // Deserialize the JSON string into a Vec<Todo>
            let todos =
                serde_json::from_str(&contents).context("Failed to parse todo file JSON")?;
            Ok(todos)
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            // If the file is not found, it's not an error.
            // Just return an empty list of todos.
            Ok(Vec::new())
        }
        Err(e) => {
            // For any other error, wrap it and return.
            Err(e).context("Failed to open todo file")
        }
    }
}

// Function to save todos to a JSON file
fn save_todos<P: AsRef<Path>>(path: P, todos: &[Todo]) -> Result<()> {
    // Open the file with write, create, and truncate options.
    // This will create the file if it doesn't exist, and clear it before writing.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .context("Failed to open or create todo file for writing")?;

    // Serialize the Vec<Todo> into a JSON string.
    let json_string =
        serde_json::to_string_pretty(todos).context("Failed to serialize todos to JSON")?;

    // Write the JSON string to the file.
    file.write_all(json_string.as_bytes())
        .context("Failed to write to todo file")?;

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new to-do item
    Add {
        /// The task description
        task: String,
    },
    /// List all to-do items
    List,
    /// Edit an existing to-do item's description
    Edit {
        /// The ID of the to-do to edit
        id: u32,
        /// The new task description
        #[arg(short, long)]
        new_task: String,
    },
    /// Mark a to-do item as complete
    Complete {
        /// The ID of the to-do to complete
        id: u32,
    },
    /// Delete a to-do item
    Delete {
        /// The ID of the to-do to delete
        id: u32,
    },
}

fn main() -> Result<()> {
    const TODO_FILE: &str = "todos.json";

    let cli = Cli::parse();
    let mut todos = load_todos(TODO_FILE)?;

    match cli.command {
        Commands::Add { task } => {
            // Find the highest existing ID and add 1 for the new ID
            let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let new_todo = Todo {
                id: new_id,
                task,
                completed: false,
            };
            println!(
                "✅ Added new to-do: \"{}\" (ID: {})",
                new_todo.task, new_todo.id
            );
            todos.push(new_todo);
        }

        Commands::List => {
            if todos.is_empty() {
                println!("No to-dos yet! Add one with the 'add' command.");
            } else {
                println!("--- Your To-Do List ---");
                for todo in todos {
                    let status = if todo.completed { "[x]" } else { "[ ]" };
                    println!("{} {}: {}", status, todo.id, todo.task);
                }
            }
            // No need to save, since we didn't change anything
            return Ok(());
        }

        Commands::Edit { id, new_task } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.task = new_task.clone();
                println!("📝 Edited to-do {}: \"{}\"", id, todo.task);
            } else {
                eprintln!("Error: To-do with ID {} not found.", id);
            }
        }

        Commands::Complete { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = true;
                println!("🎉 Completed to-do {}: \"{}\"", id, todo.task);
            } else {
                eprintln!("Error: To-do with ID {} not found.", id);
            }
        }

        Commands::Delete { id } => {
            let initial_len = todos.len();
            todos.retain(|t| t.id != id);

            if todos.len() < initial_len {
                println!("🗑️ Deleted to-do with ID {}.", id);
            } else {
                eprintln!("Error: To-do with ID {} not found.", id);
            }
        }
    }

    // Save the potentially modified list of todos back to the file
    save_todos(TODO_FILE, &todos)?;

    Ok(())
}
