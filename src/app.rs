use std::fs::{read_to_string, write};
use std::process::exit;

use crate::{
    input::{get_number, input},
    ui::clear_terminal,
};

pub struct Todo {
    pub tasks: Vec<Task>,
}

impl Todo {
    pub fn add(&mut self) {
        clear_terminal();
        let entry = Task {
            name: input("Enter the task name: "),
            deadline: input("Enter the deadline: "),
        };
        self.tasks.push(entry);
    }
    pub fn delete(&mut self) {
        clear_terminal();
        self.view();
        let mut idx = get_number("Enter a number: ").try_into().unwrap();
        idx = &idx - 1;
        if idx <= self.tasks.len() {
            self.tasks.remove(idx);
        } else {
            println!(
                "Index out of range. Enter a number between 1 and {}",
                self.tasks.len()
            );
            self.delete();
        }
    }
    pub fn edit(&mut self) {
        clear_terminal();
        self.view();

        if self.tasks.is_empty() {
            println!("No tasks to edit.");
            return;
        }

        let index_to_edit = get_number("Enter the number of the task to edit: ") as usize;
        if index_to_edit > 0 && index_to_edit <= self.tasks.len() {
            let task_to_edit = &mut self.tasks[index_to_edit - 1];
            clear_terminal();
            println!("Editing task:");
            println!("   Name: {}", task_to_edit.name);
            println!("   Deadline: {}", task_to_edit.deadline);

            println!("Select an option:");
            println!("1) Edit Name");
            println!("2) Edit Deadline");
            println!("3) Cancel");

            let option = get_number("Enter your choice: ");
            match option as u8 {
                1 => {
                    task_to_edit.name = input("Enter the new task name: ");
                    println!("Name updated successfully!");
                }
                2 => {
                    task_to_edit.deadline = input("Enter the new deadline: ");
                    println!("Deadline updated successfully!");
                }
                3 => println!("Edit canceled."),
                _ => println!("Invalid option. Edit canceled."),
            }
        } else {
            println!("Invalid task number. Edit canceled.");
        }
    }
    pub fn view(&self) {
        if !self.tasks.is_empty() {
            println!("TASKS:");
        }
        let mut index: u8 = 1;
        for entry in &self.tasks {
            println!(
                "   {}) {}\n      Due: {}",
                index, entry.name, entry.deadline
            );
            index += 1;
        }
        if !self.tasks.is_empty() {
            println!("-----------------");
        }
    }
    pub fn save_to_file(&self, file_path: String) {
        let content: Vec<String> = self
            .tasks
            .iter()
            .map(|task| format!("{}|{}", task.name, task.deadline))
            .collect();

        if let Err(err) = write(file_path, content.join("\n")) {
            eprintln!("Error saving to file: {}", err);
        }
    }
    pub fn exit_app(&self) {
        clear_terminal();
        exit(0);
    }

    pub fn read_file(&self, file_path: String) -> Vec<Task> {
        let contents = read_to_string(file_path).expect("Failed to read file");
        let mut tasks_vector: Vec<Task> = Vec::new();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() >= 2 {
                let entry = Task {
                    name: parts[0].trim().to_string(),
                    deadline: parts[1].trim().to_string(),
                };
                tasks_vector.push(entry);
            }
        }
        tasks_vector
    }
}

pub struct Task {
    name: String,
    deadline: String,
}
