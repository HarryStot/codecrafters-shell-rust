mod command;

#[allow(unused_imports)]
use std::io::{self, Write};
use crate::command::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match Command::from(command.trim()) {
            Ok(cmd) => cmd.execute(),
            Err(e) => eprintln!("{}", e),
        }
    }
}