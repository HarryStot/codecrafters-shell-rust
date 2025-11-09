#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let mut iter = command.trim().split_whitespace();

        match iter.next() {
            Some("exit") => match iter.next() {
                Some(arg) => match arg {
                    "0" => break,
                    _ => panic!(),
                },
                None => panic!(),
            },
            Some("echo") => println!("{}", iter.collect::<Vec<&str>>().join(" ")),
            Some(command) => println!("{}: command not found", command),
            None => println!("No command provided"),
        }
    }
}