use super::{Command, CommandError, Redirection};
use std::io::Write;

pub(crate) fn echo_cmd(message: &str, writer: &mut dyn Write) {
    // Write to the provided writer.
    writeln!(writer, "{}", message).unwrap();
}

pub(crate) fn parse_echo_cmd(args: &str, redirections: Vec<Redirection>) -> Result<Command, CommandError> {
    Ok(Command::Echo {
        message: args.to_owned(),
        redirections,
    })
}