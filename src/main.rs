// 1. Define the Task struct and its implementation
#[derive(Debug)]
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

// 2. Add Core Functions
fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = tasks.len() + 1;
    let task = Task::new(id, description);
    tasks.push(task);
    println!("Task added!");
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!("{}: {} [{}]", task.id, task.description, if task.completed { "Done" } else { "Pending" });
    }
}

fn complete_task(tasks: &mut Vec<Task>, id: usize) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        println!("Task {} marked as completed!", id);
    } else {
        println!("Task not found.");
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: usize) {
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        println!("Task {} deleted!", id);
    } else {
        println!("Task not found.");
    }
}

// 3. Main Function
use std::io;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("Choose an action: [1] Add Task [2] List Tasks [3] Complete Task [4] Delete Task [5] Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut description = String::new();
                println!("Enter task description:");
                io::stdin().read_line(&mut description).unwrap();
                add_task(&mut tasks, description.trim().to_string());
            }
            "2" => list_tasks(&tasks),
            "3" => {
                let mut id = String::new();
                println!("Enter task ID to complete:");
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse::<usize>() {
                    complete_task(&mut tasks, id);
                }
            }
            "4" => {
                let mut id = String::new();
                println!("Enter task ID to delete:");
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse::<usize>() {
                    delete_task(&mut tasks, id);
                }
            }
            "5" => break,
            _ => println!("Invalid choice!"),
        }
    }
}
