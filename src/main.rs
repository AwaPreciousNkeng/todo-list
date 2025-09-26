use std::io::{stdin, stdout, Write};
use todo::Task;

fn run_prompt(todo: &mut Vec<Task>) {
    loop {
        let mut stdout = stdout().lock();
        print!("(PCodes-List) > ");
        stdout.flush().expect("Could not flush stdout");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Could not read line");

        //Take the args into the run function of lib and get the result of the computation out.
        let command: &str = buffer.split_whitespace().collect::<Vec<&str>>()[0];
        let task_string = concatenate_task_string(buffer.split_whitespace().collect());
        let args = vec![command, task_string.as_str()];
        todo::run(args, todo);
    }
}

fn concatenate_task_string(word_vec: Vec<&str>) -> String {
    let mut result = String::new();
    // We are assuming that everything after the first word is part of task string.
    for i in 1..word_vec.len() {
        result = result + word_vec[i];
        result = result + " ";
    }
    result
}

fn main() {
    let mut todo: Vec<Task> = Vec::new();
    run_prompt(&mut todo);
}


