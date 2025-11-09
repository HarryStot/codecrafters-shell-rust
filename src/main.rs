#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();

        println!("{}: command not found", command.trim());
    }
}
