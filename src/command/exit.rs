use super::{Command, CommandError};

pub(crate) fn exit_cmd(code: i32) {
    std::process::exit(code);
}

pub(crate) fn parse_exit_cmd(args: &str) -> Result<Command, CommandError> {
    let value = match args.is_empty() {
        true => 0,
        false => args.parse::<i32>().unwrap_or(1),
    };
    Ok(Command::Exit(value))
}