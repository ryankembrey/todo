mod app;
mod input;
mod ui;
use app::Todo;
use clap::{App, Arg};
use input::get_number;
use std::fs::File;
use ui::clear_terminal;

fn main() {
    let matches = App::new("Todo App")
        .version("1.0")
        .about("A simple todo list application.")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Sets the file to store todos")
                .takes_value(true),
        )
        .get_matches();

    let file_path = matches
        .value_of("file")
        .map_or_else(|| String::from("todo.txt"), String::from);

    // Ensure the file exists or create it
    if !File::open(&file_path).is_ok() {
        if let Err(err) = File::create(&file_path) {
            eprintln!("Error creating file: {}", err);
            return;
        }
    }

    // Read the tasks from the file
    let mut my_todo = Todo { tasks: Vec::new() };
    my_todo.tasks = my_todo.read_file(file_path.clone());

    loop {
        clear_terminal();
        my_todo.view();
        println!("Actions:\n   1) Add\n   2) Delete\n   3) Edit\n   4) Exit");
        println!("Current file path: {}", file_path);
        let action = get_number("Enter a number: ");
        match action as u8 {
            1 => my_todo.add(),
            2 => my_todo.delete(),
            3 => my_todo.edit(),
            4 => {
                // Save the tasks to the file before exiting
                my_todo.save_to_file(file_path.clone());
                my_todo.exit_app();
            }
            _ => println!("Invalid input"),
        }
    }
}
