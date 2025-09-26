use std::process;
use std::sync::atomic;
use std::sync::atomic::AtomicU64;

#[derive(Debug)]
pub struct Task {
    pub task: String,
    pub is_completed: bool,
    pub id: u64
}

impl Task {
    fn update_status(&mut self) {
        self.is_completed = true;
    }

    fn update_task(&mut self, new_name: &str) {
        self.task = new_name.to_string();
    }
}

static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);

fn display_todo (todo_list: &Vec<Task>) {
    if todo_list.len() == 0 {
        println!("Empty Todo list");
        return;
    }

    for task in todo_list {
        println!("id: {}, name: {}, completed: {}", task.id, task.task, task.is_completed);
    }
}

fn add_new_task(todo_list: &mut Vec<Task>, task_name: &str) {
    let id = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);

    let task: Task = Task {
        task: task_name.to_string(),
        is_completed: false,
        id
    };
    todo_list.push(task);
    println!("{} added to the todo list", task_name);
}

fn remove_task(todo_list: &mut Vec<Task>, id: u64) {
    todo_list.retain(|t| t.id != id);
}

fn get_task(todo_list: &mut Vec<Task>, id: u64) -> Option<&mut Task> {
    todo_list.iter_mut().find(|task| task.id == id)
    }

fn display_help() {
    let help: &str = "
        Welcome to the todo_list application.
        structure of query:
            command [arguments]

        supported commands:
            add - Add a new task to the todo list, followed by a new task string. The task string should NOT be space separated.

                usage: >add task_string

            show - Display the todo list

                usage: >show

            delete - delete a task from the todo list, based on the task id provided by the user in the prompt.

                usage: >delete task_id

            update - change the name of a task, followed by an integer number task id.

                usage: >update task_id new_task_string

            done - change the done status of a task from false to true, follwed by an integer number task id.

                usage: >done task_id

            exit- exit the program.

                usage: >exit

            help - display this help message.

                usage: >help

        arguments:
            task_id: the unique id assigned to each task.

            task_string: the string for the task provided by the user. ";

    println!("{}", help);
}

fn parse_args(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    if args.is_empty() {
        println!("No commands provided");
    }
    let command = args[0];

    match command {
        "add" => {
            if let Some(value) = args.get(1) {
                let new_task = *value;
                add_new_task(todo_list, new_task);
                display_todo(todo_list);
            } else {
                println!("Please specify a task name");
            }
        },
        "show" => {
            display_todo(todo_list);
        },
        "delete" => {
            if let Some(id_str) = args.get(1) {
                match id_str.parse::<u64>() {
                    Ok(id) => {
                        remove_task(todo_list, id);
                    }

                    Err(message) => {
                        println!("Invalid Id: {}", message);
                    }
                }
            }
        },

        "update" => {
            if let Some(id_str) = args.get(1) {
                match id_str.parse::<u64>() {
                    Ok(id) => {
                        if let Some(task) = get_task(todo_list, id) {
                            if let Some(value) = args.get(2) {
                                task.update_task(value);
                                println!("Updated Task: {}", id);
                            } else {
                                println!("No new task provided");
                            }
                        } else {
                            println!("Task not found in todo list");
                        }
                    },
                    Err(message) => {
                        println!("Invalid Id: {}", message);
                    }
                }
            }
        }
        "done" => {
            if let Some(id_str) = args.get(1) {
                match id_str.parse::<u64>() {
                    Ok(id) => {
                        if let Some(task) = get_task(todo_list, id) {
                            task.update_status();
                        } else {
                            println!("Task Id not found in the list");
                        }
                    },
                    Err(message) => {
                        println!("Invalid Id: {}", message);
                    }
                }
            }
        }
        "exit" => {
            process::exit(0);
        }
        "help" |_ => {
            display_help();
        }
    }
}

pub fn run(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    parse_args(args, todo_list);
}
