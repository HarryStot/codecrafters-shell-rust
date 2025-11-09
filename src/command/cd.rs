use std::path::Path;
use std::env::set_current_dir;
use super::{Command, CommandError};

pub(crate) fn cd_cmd(args: &str) {
    // Absolute path
    let path = Path::new(args);

    match set_current_dir(path) {
        Ok(_t) => (),
        Err(_e) => eprintln!("cd: {}: No such file or directory", path.display()),
    }
}

pub(crate) fn parse_cd_cmd(args: &str) -> Result<Command, CommandError> {
    Ok(Command::Cd(args.to_owned()))
}