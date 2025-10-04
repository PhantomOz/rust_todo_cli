# Rust CLI To-Do App

A simple, fast, and persistent command-line to-do application built with Rust. Manage your tasks directly from the terminal without ever leaving your workflow.

The to-do list is saved locally in a `todos.json` file in the same directory where you run the application.

## Demo

Here is a quick demonstration of the application's workflow:

```bash
# Add a few tasks
$ cargo run -- add "Learn Rust basics"
✅ Added new to-do: "Learn Rust basics" (ID: 1)

$ cargo run -- add "Build a CLI app"
✅ Added new to-do: "Build a CLI app" (ID: 2)

# List current tasks
$ cargo run -- list
--- Your To-Do List ---
[ ] 1: Learn Rust basics
[ ] 2: Build a CLI app

# Mark the first task as complete
$ cargo run -- complete 1
🎉 Completed to-do 1: "Learn Rust basics"

# List again to see the change
$ cargo run -- list
--- Your To-Do List ---
[x] 1: Learn Rust basics
[ ] 2: Build a CLI app

# Edit the second task
$ cargo run -- edit 2 --new-task "Build an awesome CLI app in Rust"
📝 Edited to-do 2: "Build an awesome CLI app in Rust"

# Delete the first task
$ cargo run -- delete 1
🗑️ Deleted to-do with ID 1.

# Final list
$ cargo run -- list
--- Your To-Do List ---
[ ] 2: Build an awesome CLI app in Rust
```

## Features

  - **Add** new tasks.
  - **List** all tasks with their completion status.
  - **Edit** the description of existing tasks.
  - **Complete** tasks by marking them as done.
  - **Delete** tasks from the list.
  - **Persistent Storage**: Your tasks are automatically saved to `todos.json`.

## Prerequisites

You need to have the Rust toolchain (including `rustc` and `cargo`) installed. You can get it from [rustup.rs](https://rustup.rs/).

## Installation & Usage

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/PhantomOz/rust_todo_cli.git
    cd rust_todo_cli
    ```

2.  **Build the project:**
    For a development build, use:

    ```bash
    cargo build
    ```

    For an optimized release build, use:

    ```bash
    cargo build --release
    ```

3.  **Run the application:**

    You can run the app through Cargo (easiest for development):

    ```bash
    cargo run -- <COMMAND>
    ```

    Or, you can run the compiled binary directly (from the `target/release` directory if you built for release):

    ```bash
    ./target/release/rust_todo_cli <COMMAND>
    ```

    *(For convenience, you could move this binary to a directory in your system's PATH, like `/usr/local/bin`)*

## Commands

### `add`

Adds a new, uncompleted task to your to-do list.

  - **Usage:** `cargo run -- add "<task description>"`
  - **Example:** `cargo run -- add "Buy groceries"`

### `list`

Displays all tasks, showing their ID, completion status, and description.

  - **Usage:** `cargo run -- list`

### `edit`

Changes the description of an existing task, identified by its ID.

  - **Usage:** `cargo run -- edit <ID> --new-task "<new task description>"`
  - **Example:** `cargo run -- edit 2 --new-task "Buy milk and bread"`

### `complete`

Marks a task as complete, identified by its ID.

  - **Usage:** `cargo run -- complete <ID>`
  - **Example:** `cargo run -- complete 1`

### `delete`

Permanently removes a task from the list, identified by its ID.

  - **Usage:** `cargo run -- delete <ID>`
  - **Example:** `cargo run -- delete 3`

## Future Improvements

This is a simple implementation with room for more features:

  - Store `todos.json` in a more conventional user directory (e.g., `~/.config/rust_todo_cli/`).
  - Add sub-commands to un-complete a task or clear the entire list.
  - Implement priority levels or due dates for tasks.
  - Add more colorful and interactive output.

## License

This project is licensed under the MIT License.