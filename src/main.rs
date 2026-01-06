use std::{env, fs::File, io::{Read, Write}};
use std::fmt::Display;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use owo_colors::OwoColorize;



#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    description: String,
    time_created: DateTime<Local>
}

impl Task {
    fn new(name: String, description: String) -> Self {
        Self { name, description, time_created: Local::now()}
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n\t{}", self.name.purple().bold().underline(), self.description, self.time_created.format("%d/%m/%Y %H:%M").to_string().bright_black().italic())
    }
}


#[derive(Serialize, Deserialize)]
struct ToDo {
    tasks: Vec<Task>
}

impl ToDo {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn get(&mut self, num: u32) -> &mut Task {
        self.tasks.get_mut(num as usize).unwrap()
    }

    fn pop(&mut self, num: u32) -> Task {
        self.tasks.remove(num as usize)
    }
}

impl Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str_tasks: Vec<String> = vec![];
        for (num, task) in self.tasks.iter().clone().enumerate() {
            str_tasks.push(format!("{num}) {task}"));
        }

        write!(f, "{}", &str_tasks.join("\n\n"))
    }
}


fn main() {
    let path = env::current_exe().unwrap().parent().unwrap().join("./tasks.json").canonicalize().unwrap();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            let mut file = File::create(&path).unwrap();
            file.write_all(to_string_pretty(&ToDo::new()).unwrap().as_bytes()).unwrap();
            File::open(&path).unwrap()
        }
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut todo = from_str::<ToDo>(&data).unwrap();

    let args: Vec<String> = env::args().collect();
    let command = &args[1].to_lowercase();
    match command.as_str() {
        "add" | "a" => {
            let task = Task::new(
                args.get(2)
                    .expect("Name must be given")
                    .clone(),
                {
                    args.get(3..).unwrap().join(" ")
                }
            );
            println!("{}) {task} was sucesfully added", todo.tasks.len());
            todo.add(task);
        }

        "change" | "ch" | "c" => {
            let num: u32 = args[2].parse().expect("First argument of change should be number");
            let task = todo.get(num);
            match args[3].as_str() {
                "name" | "n" => task.name = args[4..].join(" "),
                "description" | "desc" | "d" => task.description = args[4..].join(" "),
                _ => panic!("Second argument of change should be name or description")
            }
        }

        "list" | "l" => println!("{todo}"),

        "remove" | "rm" | "r" => {
            let num: u32 = args.get(2).expect("Task's number must be given").parse().unwrap();
            let task = todo.pop(num);
            println!("{task} was sucessfully removed");
        }

        "file" => println!("{}", path.display()),

        _ => println!("Wrong command")
    }


    let data = to_string_pretty(&todo).unwrap();
    File::create(&path).unwrap().write_all(data.as_bytes()).unwrap();
}