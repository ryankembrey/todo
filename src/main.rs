mod app;
mod input;
mod ui;

use app::Todo;
use input::get_number;
use ui::clear_terminal;

fn main() {
    let file_path = String::from("todo.txt");

    // Read the tasks from the file
    let mut my_todo = Todo { tasks: Vec::new() };
    my_todo.tasks = my_todo.read_file(file_path.clone());

    loop {
        clear_terminal();
        my_todo.view();
        println!("Actions:\n   1) Add\n   2) Delete\n   3) Edit\n   4) Exit");
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
