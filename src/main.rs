use std::io::{stdin, stdout, Write};
use todo::{load_tasks, run, Task};

fn run_prompt(todo_list: &mut Vec<Task>) {
    loop {
        let mut stdout = stdout();
        print!("(PCodes-TODO) > ");
        stdout.flush().expect("Could not flush stdout");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Could not read line");

        let buffer = buffer.trim();
        if buffer.is_empty() {
            continue;
        }

        // Split input into words
        let words: Vec<&str> = buffer.split_whitespace().collect();
        println!("{:?}", words);
        let command = words[0].to_string();
        println!("{:?}", command);

        // Everything after the first word is treated as a single argument (multi-word task)
        let task_string = if words.len() > 1 {
            words[1..].join(" ")
        } else {
            String::new()
        };

        let args = vec![command, task_string];
        println!("{:?}", args);
        run(args, todo_list);
    }
}

fn main() {
    let mut todo_list = load_tasks().unwrap_or_default();
    run_prompt(&mut todo_list);
}
