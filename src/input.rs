use std::io;

pub fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input.trim().to_string()
}

pub fn get_number(prompt: &str) -> i8 {
    let user_input = input(prompt);
    match user_input.trim().parse() {
        Ok(num) if num >= 0 => num,
        Ok(_) => {
            println!("Please enter a non-negative number.");
            0
        }
        Err(_) => {
            println!("{} is not a number.", user_input.trim());
            0
        }
    }
}
