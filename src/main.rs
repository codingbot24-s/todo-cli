use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

const FILE_NAME: &str = "todo.json";
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}
#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A simple command-line todo app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Remove { id: u32 },
    Done { id: u32 },
}

fn load_task() -> Vec<Task> {
    if !Path::new(FILE_NAME).exists() {
        return Vec::new();
    }

    let mut file = File::open(FILE_NAME).expect("Error opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("error reading file");

    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_task(tasks: &Vec<Task>) {
    let contents = serde_json::to_string_pretty(tasks).expect("Error converting");
    let mut file = File::create(FILE_NAME).expect("error opening file");
    file.write_all(contents.as_bytes())
        .expect("errror writing file");
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_task();

    match cli.command {
        Commands::Add { description } => {
            let id = if tasks.is_empty() {
                1
            } else {
                tasks.last().unwrap().id + 1
            };
            let task = Task {
                id,
                description,
                done: false,
            };
            tasks.push(task);
            save_task(&mut tasks);
            println!("Task Saved");
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                println!("Task List");
                for task in &tasks {
                    println!(
                        "{}. [{}] {}",
                        task.id,
                        if task.done { "x" } else { " " },
                        task.description
                    );
                }
            }
        }
        Commands::Remove { id } => {
            let original_len = tasks.len();
            tasks.retain(|task| task.id != id);

            if tasks.len() < original_len {
                save_task(&tasks);
                println!("Removed task successfully ");
            } else {
                println!("⚠️ Task with ID {} not found.", id);
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_task(&tasks);
                println!("Task mark as done")
            } else {
                println!("Error Marking As Done")
            }
        }
    }
}
