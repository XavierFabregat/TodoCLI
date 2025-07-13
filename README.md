# Rust Todo CLI

A simple, fast, and robust todo CLI tool written in Rust, using SQLite for local storage.

## Features

- Add, list, update, complete, and delete tasks
- Tasks have a title, optional description, due date, priority, and completion status
- Due dates must be in the future (validated)
- Priorities: low, medium, high
- Colorful terminal output
- All data stored locally in a SQLite database (`~/.todo.db`)
- Fully tested with unit and integration tests

## Installation

1. **Clone the repository:**
   ```sh
   git clone <your-repo-url>
   cd todo
   ```
2. **Build the project:**
   ```sh
   cargo build --release
   ```
3. **(Optional) Install globally:**
   ```sh
   cargo install --path .
   ```

## Usage

```
A simple todo CLI tool with SQLite storage

Usage: todo <COMMAND>

Commands:
  add       Add a new task
  list      List all tasks
  complete  Mark a task as completed
  delete    Delete a task
  update    Update a task
  show      Show details of a specific task
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Examples

- **Add a task:**
  ```sh
  todo add "Buy groceries" --description "Milk, bread, eggs" --due 2024-12-31 --priority high
  ```
- **List tasks:**
  ```sh
  todo list
  ```
- **List only completed tasks:**
  ```sh
  todo list --completed
  ```
- **List only high priority tasks:**
  ```sh
  todo list --priority high
  ```
- **Complete a task:**
  ```sh
  todo complete 1
  ```
- **Update a task:**
  ```sh
  todo update 1 --title "Buy groceries and snacks" --priority medium
  ```
- **Delete a task:**
  ```sh
  todo delete 1
  ```
- **Show task details:**
  ```sh
  todo show 1
  ```

## Development & Testing

- **Run all tests:**
  ```sh
  cargo test
  ```
- **Run with coverage (requires [cargo-tarpaulin](https://github.com/xd009642/tarpaulin))**
  ```sh
  cargo tarpaulin --out Html
  ```
- **Check formatting:**
  ```sh
  cargo fmt -- --check
  ```

## Project Structure

- `src/main.rs` — CLI entry point and argument parsing
- `src/commands.rs` — Command implementations
- `src/db.rs` — SQLite database logic
- `src/models.rs` — Task model and display logic
- `tests/` — Integration tests

## Contributing

Pull requests and issues are welcome! Please add tests for new features.

## License

MIT
