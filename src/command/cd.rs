use std::path::Path;
use super::{Command, CommandError};

pub(crate) fn cd_cmd(args: &str) {
    // Absolute path
    let path = Path::new(args);
}

pub(crate) fn parse_cd_cmd(args: &str) -> Result<Command, CommandError> {
    Ok(Command::Cd(args.to_owned()))
}