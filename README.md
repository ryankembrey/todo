# Simple ToDo List App

A minimalistic to-do list application written in Rust.

## Features

- Add tasks with names and deadlines.
- View tasks with their respective deadlines.
- Delete tasks by selecting their numbers.
- Edit tasks to update names or deadlines.
- Specify the file path directly as the first command-line argument for more flexibility in task storage.

## Getting Started

### Prerequisites

- Rust programming language installed. [Install Rust](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/todo.git ; cd todo-app
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Run the application:

    ```bash
    ./target/release/todo [optional_file_path]
    ```

   If no file path is provided, the application will default to "todo.txt" in the current directory.

   Alternatively, download a binary from the [releases](https://github.com/ryankembrey/todo/releases) page.

## Usage

- Upon running the app, you can interact with the to-do list through the terminal.
- Follow the on-screen instructions to perform actions such as adding, viewing, deleting, or editing tasks.

## Contributing

Contributions are welcome! Feel free to open issues or pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
