use super::{Command, CommandError};

pub(crate) fn echo_cmd(args: &str) {
    println!("{}", args);
}

pub(crate) fn parse_echo_cmd(args: &str) -> Result<Command, CommandError> {
    Ok(Command::Echo(args.to_owned()))
}