#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        if command == "exit 0" {
            break;
        } else if !command.is_empty() {
            println!("{}: command not found", command);
        }
    }
}
