# Rust Todo App

A **command-line based Todo App** built with Rust, designed for managing tasks per user. It features persistent storage using JSON files, making it simple yet effective for tracking your tasks and progress.

---

## Features 🚀

- **User Management**:
  - Create and switch between users.
  - Each user has their own task list.

- **Task Management**:
  - Add tasks with a description.
  - List all tasks with their status (Pending/Done).
  - Mark tasks as completed.
  - Delete tasks.

- **Data Persistence**:
  - Tasks are saved in a `tasks.json` file.
  - Automatically loads tasks when the app starts.

---

## Demo 🎥

### Start the App:
```bash
cargo run
```

### Menu Options:
1. **Select User**:
   - Create a new user or switch to an existing one.
2. **Add Task**:
   - Add a new task for the selected user.
3. **List Tasks**:
   - View all tasks for the current user.
4. **Complete Task**:
   - Mark a task as completed.
5. **Delete Task**:
   - Remove a task from the user's list.
6. **Exit**:
   - Save all tasks and exit the application.

---

## Installation 🛠️

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/Usamahafiz8/rust-todo-app
   cd rust-todo-app
   ```

2. **Install Rust** (if not already installed):
   - Follow the official instructions: [Rust Installation](https://www.rust-lang.org/tools/install)

3. **Run the App**:
   ```bash
   cargo run
   ```

---

## File Structure 📁

```plaintext
todo_app/                 # Root project directory
├── src/                 # Source code directory
│   └── main.rs          # Main application logic
├── tasks.json           # JSON file for storing tasks (auto-generated)
├── Cargo.toml           # Rust project configuration file
├── Cargo.lock           # Dependency lock file (auto-generated)
```

---

## Example JSON File 📂

After adding some users and tasks, your `tasks.json` file might look like this:

```json
{
  "Alice": [
    {
      "id": 1,
      "description": "Buy groceries",
      "completed": false
    },
    {
      "id": 2,
      "description": "Complete Rust project",
      "completed": true
    }
  ],
  "Bob": [
    {
      "id": 1,
      "description": "Clean the house",
      "completed": false
    }
  ]
}
```

---

## Dependencies 📦

The following crates are used in this project:

- [serde](https://crates.io/crates/serde) - For data serialization and deserialization.
- [serde_json](https://crates.io/crates/serde_json) - For working with JSON files.

---

## Contribution 🤝

Contributions are welcome! If you have any suggestions or want to add features, feel free to create a pull request or open an issue.

---

## License 📜

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments 🙌

Thanks to the amazing Rust community for the awesome resources and support!

---

### Let's build something great together! 🚀
```

---

Let me know if you'd like any changes or if there's a specific style you'd prefer!