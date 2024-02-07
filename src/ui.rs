use std::process::Command;

pub fn clear_terminal() {
    if cfg!(unix) {
        Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    } else if cfg!(windows) {
        Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status()
            .expect("Failed to clear terminal");
    }
}
