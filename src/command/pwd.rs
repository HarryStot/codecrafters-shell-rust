use std::env::current_dir;
use super::{Command, CommandError};

pub(crate) fn pwd_cmd() {
    match current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                println!("{}", path_str);
            } else {
                eprintln!("pwd: unable to convert path to string");
            }
        }
        Err(e) => {
            eprintln!("pwd: error retrieving current directory: {}", e);
        }
    }
}

pub(crate) fn parse_pwd_cmd(p0: &str) -> Result<Command, CommandError> {
    if p0.trim().is_empty() {
        Ok(Command::Pwd)
    } else {
        Err(CommandError::InvalidArguments("pwd".to_string()))
    }
}