use super::{Command, CommandError, Redirection, RedirectionTarget, utils};
use std::io::Write;

pub(crate) fn type_cmd(cmd: &str, redirections: &[Redirection]) {
    use Command::*;
    if cmd.is_empty() {
        return;
    }

    let mut stdout_writer = utils::get_output_writer(redirections, RedirectionTarget::Stdout);
    let mut stderr_writer = utils::get_output_writer(redirections, RedirectionTarget::Stderr);

    // We need to parse the command without executing it, so we create a new `from` without redirections.
    let cmd_to_check = format!("{} {}", cmd, "");
    match Command::from(&cmd_to_check) {
        Ok(External { path, .. }) => {
            writeln!(stdout_writer, "{} is {}", cmd, path).unwrap();
        }
        Ok(Noop) => {
            // This case can be hit if the input to `type` is just a redirection, which is not a valid command.
            writeln!(stderr_writer, "{}: not found", cmd).unwrap();
        }
        Ok(_) => {
            writeln!(stdout_writer, "{} is a shell builtin", cmd).unwrap();
        }
        Err(CommandError::NotFound(..)) => {
            writeln!(stderr_writer, "{}: not found", cmd).unwrap();
        }
        Err(e) => {
            writeln!(stderr_writer, "type: error: {}", e).unwrap();
        }
    };
}

pub(crate) fn parse_type_cmd(args: &str, redirections: Vec<Redirection>) -> Result<Command, CommandError> {
    let cmd = args.trim().to_owned();
    Ok(Command::Type { cmd, redirections })
}