use super::{Command, CommandError, Redirection, RedirectionTarget, utils};
use std::io::Write;

pub(crate) fn echo_cmd(message: &str, redirections: &[Redirection]) {
    // Get the writer for stdout. It will be the file or stdout itself.
    let mut writer = utils::get_output_writer(redirections, RedirectionTarget::Stdout);

    // Write to the returned writer.
    writeln!(writer, "{}", message).unwrap();
}

pub(crate) fn parse_echo_cmd(args: &str, redirections: Vec<Redirection>) -> Result<Command, CommandError> {
    Ok(Command::Echo {
        message: args.to_owned(),
        redirections,
    })
}