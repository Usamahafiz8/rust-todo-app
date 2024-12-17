use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;

// 1. Define the Task struct and its implementation
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }
}

// 2. Core Functions for Task Management
fn add_task(users: &mut HashMap<String, Vec<Task>>, username: &str, description: String) {
    let tasks = users.entry(username.to_string()).or_insert(Vec::new());
    let id = tasks.len() + 1;
    let task = Task::new(id, description);
    tasks.push(task);
    println!("Task added!");
}

fn list_tasks(users: &HashMap<String, Vec<Task>>, username: &str) {
    if let Some(tasks) = users.get(username) {
        if tasks.is_empty() {
            println!("No tasks found for user: {}", username);
        } else {
            for task in tasks {
                println!(
                    "{}: {} [{}]",
                    task.id,
                    task.description,
                    if task.completed { "Done" } else { "Pending" }
                );
            }
        }
    } else {
        println!("No tasks found for user: {}", username);
    }
}

fn complete_task(users: &mut HashMap<String, Vec<Task>>, username: &str, id: usize) {
    if let Some(tasks) = users.get_mut(username) {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task {} marked as completed!", id);
        } else {
            println!("Task not found.");
        }
    } else {
        println!("User not found.");
    }
}

fn delete_task(users: &mut HashMap<String, Vec<Task>>, username: &str, id: usize) {
    if let Some(tasks) = users.get_mut(username) {
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(pos);
            println!("Task {} deleted!", id);
        } else {
            println!("Task not found.");
        }
    } else {
        println!("User not found.");
    }
}

// 3. File Persistence Functions
fn save_to_file(users: &HashMap<String, Vec<Task>>) {
    if let Ok(json) = serde_json::to_string_pretty(users) {
        fs::write("tasks.json", json).expect("Failed to save tasks.");
    }
}

fn load_from_file() -> HashMap<String, Vec<Task>> {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        if let Ok(users) = serde_json::from_str(&data) {
            return users;
        }
    }
    HashMap::new()
}

// 4. Main Function
fn main() {
    let mut users: HashMap<String, Vec<Task>> = load_from_file();
    let mut current_user = String::new();

    loop {
        println!(
            "Choose an action: [1] Select User [2] Add Task [3] List Tasks [4] Complete Task [5] Delete Task [6] Exit"
        );
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter username:");
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim().to_string();
                if !users.contains_key(&username) {
                    users.insert(username.clone(), Vec::new());
                    println!("New user created!");
                }
                current_user = username;
                println!("Switched to user: {}", current_user);
            }
            "2" => {
                if current_user.is_empty() {
                    println!("Select a user first!");
                } else {
                    let mut description = String::new();
                    println!("Enter task description:");
                    io::stdin().read_line(&mut description).unwrap();
                    add_task(&mut users, &current_user, description.trim().to_string());
                }
            }
            "3" => {
                if current_user.is_empty() {
                    println!("Select a user first!");
                } else {
                    list_tasks(&users, &current_user);
                }
            }
            "4" => {
                if current_user.is_empty() {
                    println!("Select a user first!");
                } else {
                    let mut id = String::new();
                    println!("Enter task ID to complete:");
                    io::stdin().read_line(&mut id).unwrap();
                    if let Ok(id) = id.trim().parse::<usize>() {
                        complete_task(&mut users, &current_user, id);
                    }
                }
            }
            "5" => {
                if current_user.is_empty() {
                    println!("Select a user first!");
                } else {
                    let mut id = String::new();
                    println!("Enter task ID to delete:");
                    io::stdin().read_line(&mut id).unwrap();
                    if let Ok(id) = id.trim().parse::<usize>() {
                        delete_task(&mut users, &current_user, id);
                    }
                }
            }
            "6" => {
                save_to_file(&users);
                println!("Tasks saved. Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}
