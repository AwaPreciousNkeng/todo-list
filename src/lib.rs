use std::{fs, io, process};
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub task: String,
    pub is_completed: bool,
    pub id: u64,
}

impl Task {
    pub fn update_status(&mut self) {
        self.is_completed = true;
    }

    pub fn update_task(&mut self, new_name: &str) {
        self.task = new_name.to_string();
    }
}

static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);

fn default_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".todo_tasks.json");
    path
}

pub fn save_tasks(todo_list: &Vec<Task>) -> io::Result<()> {
    let path = default_path();
    let data = serde_json::to_string_pretty(todo_list)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(path, data)?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let path = default_path();
    match fs::read_to_string(path) {
        Ok(data) => Ok(serde_json::from_str(&data).unwrap_or_default()),
        Err(_) => Ok(Vec::new()),
    }
}

fn display_todo(todo_list: &Vec<Task>) {
    if todo_list.is_empty() {
        println!("Empty Todo list");
        return;
    }

    for task in todo_list {
        println!("id: {}, name: {}, completed: {}", task.id, task.task, task.is_completed);
    }
}

fn add_new_task(todo_list: &mut Vec<Task>, task_name: &str) {
    let id = UNIQUE_ID.fetch_add(1, Ordering::SeqCst);
    let task = Task {
        task: task_name.to_string(),
        is_completed: false,
        id,
    };
    todo_list.push(task);
    save_tasks(todo_list).unwrap();
    println!("Task added: {}", task_name);
}

fn remove_task(todo_list: &mut Vec<Task>, id: u64) {
    if todo_list.iter().any(|t| t.id == id) {
        todo_list.retain(|t| t.id != id);
        save_tasks(todo_list).unwrap();
        println!("Task {} deleted", id);
    } else {
        println!("Task Id not found: {}", id);
    }
}

fn get_task(todo_list: &mut Vec<Task>, id: u64) -> Option<&mut Task> {
    todo_list.iter_mut().find(|task| task.id == id)
}

fn display_help() {
    println!(
        "
Todo List Application

Commands:
  add <task_name>      - Add a new task (multi-word names allowed)
  show                 - Show all tasks
  delete <task_id>     - Delete a task by ID
  update <id> <name>   - Update task name
  done <task_id>       - Mark task as completed
  exit                 - Exit application
  help                 - Show this help
"
    );
}

// Main parsing function
pub fn run(args: Vec<String>, todo_list: &mut Vec<Task>) {
    if args.is_empty() {
        println!("No command provided.");
        return;
    }

    match args[0].as_str() {
        "add" => {
            if args.len() > 1 {
                add_new_task(todo_list, &args[1]);
                display_todo(todo_list);
            } else {
                println!("Please provide a task name");
            }
        }
        "show" => display_todo(todo_list),
        "delete" => {
            if let Some(id_str) = args.get(1) {
                match id_str.parse::<u64>() {
                    Ok(id) => remove_task(todo_list, id),
                    Err(_) => println!("Invalid Id: {}", id_str),
                }
            } else {
                println!("Please provide a task id");
            }
        }
        "update" => {
            if args.len() > 2 {
                let id_str = &args[1];
                let new_name = &args[2];
                match id_str.parse::<u64>() {
                    Ok(id) => {
                        if let Some(task) = get_task(todo_list, id) {
                            task.update_task(new_name);
                            save_tasks(todo_list).unwrap();
                            println!("Task {} updated", id);
                        } else {
                            println!("Task Id not found: {}", id);
                        }
                    }
                    Err(_) => println!("Invalid Id: {}", id_str),
                }
            } else {
                println!("Usage: update <id> <new_task_name>");
            }
        }
        "done" => {
            if let Some(id_str) = args.get(1) {
                match id_str.parse::<u64>() {
                    Ok(id) => {
                        if let Some(task) = get_task(todo_list, id) {
                            task.update_status();
                            save_tasks(todo_list).unwrap();
                            println!("Task {} marked done", id);
                        } else {
                            println!("Task Id not found: {}", id);
                        }
                    }
                    Err(_) => println!("Invalid Id: {}", id_str),
                }
            } else {
                println!("Please provide a task id");
            }
        }
        "exit" => process::exit(0),
        "help" | _ => display_help(),
    }
}
