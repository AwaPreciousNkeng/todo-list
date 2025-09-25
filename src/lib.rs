use std::{env, io};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use colored::Colorize;

pub struct Entry {
    pub todo_entry: String,
    pub done: bool
}

impl Entry {
    pub fn new(todo_entry: String, done: bool) -> Self {
        Self {
            todo_entry,
            done
        }
    }

    pub fn file_line(&self) -> String {
        let symbol = if self.done { "[*] " } else { "[ ]" };
        format!("{}{}\n", symbol, self.todo_entry)
    }

    pub fn list_line(&self, number: usize) -> String {
        //Checks if the current task is completed or not...
        let todo_entry = if self.done {
            //DONE
            //If the task is completed, then it prints it as it is
            self.todo_entry.strikethrough().to_string()
        } else {
            //Not Done
            //If the task is not completed yet, then it prints it with a strikethrough
            self.todo_entry.clone()
        };
        format!("{} {}\n", number, self.todo_entry)
    }

    pub fn read_line(line: &String) -> Self {
        let done = &line[..4] == "[*] ";
        let todo_entry = (&line[4..]).to_string();
        Self {
            todo_entry,
            done
        }
    }

    pub fn raw_line(&self) -> String {
        format!("{}\n", self.todo_entry)
    }
}

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_bak: String,
    pub no_backup: bool
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                let home = env::var("HOME").unwrap();

                //Look for a legacy TODO file path
                let legacy_todo = format!("{}/TODO", &home);
                match Path::new(&legacy_todo).exists() {
                    true => legacy_todo,
                    false => format!("{}/.todo", &home)
                }
            }
        };

        let todo_bak: String = match env::var("TODO_BAK_DIR") {
            Ok(t) => t,
            Err(_) => String::from("/tmp/todo.bak")
        };

        let no_backup = env::var("TODO_NOBACKUP").is_ok();
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect("failed to open todo file");

        //Creates a new buf reader
        let mut buf_reader = BufReader::new(&todofile);

        //Empty String ready to be filled with TODOs
        let mut contents = String::new();

        //Loads "contents" String with data
        buf_reader.read_to_string(&mut contents).unwrap();

        //Splits contents of the TODO file into a todo vector
        let todo = contents.lines().map(str::to_string).collect();

        //Returns todo
        Ok(Self {
            todo,
            todo_path,
            todo_bak,
            no_backup
        })
    }

    //Prints every todo saved
    pub fn list(&self) {
        let stdout = io::stdout();
        //Buffered writer for stdout stream
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();
        //This loop will repeat itself for each task in TODO file
        for (number, task) in self.todo.iter().enumerate() {
            let entry = Entry::read_line(task);
            let number = number + 1;
            let line = entry.list_line(number);
            data.push_str(&line);
        }
        writer
            .write_all(data.as_bytes())
            .expect("failed to write to stdout");
    }

    pub fn raw(&self, arg: &[String]) {
        if arg.len() > 1 {
            eprintln!("Todo raw takes only 1 argument, not {}", arg.len());
        } else if arg.is_empty() {
            eprintln!("Todo raw takes one argument (done/todo");
        } else {
            let stdout = io::stdout();
            //Buffered writer for stdout stream
            let mut writer = BufWriter::new(stdout);
            let mut data = String::new();
            let arg = &arg[0];
            //This loop will repeat itself for each task in todo file
            for task in self.todo.iter() {
                let entry = Entry::read_line(task);
                if entry.done && arg == "done" {

                }
            }
        }
    }
}
