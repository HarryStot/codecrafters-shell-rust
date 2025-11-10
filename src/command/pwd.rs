use std::env::current_dir;
use std::io::Write;
use super::{Command, CommandError, Redirection, RedirectionTarget, utils};

pub(crate) fn pwd_cmd(redirections: &[Redirection]) {
    let mut stdout_writer = utils::get_output_writer(redirections, RedirectionTarget::Stdout);
    let mut stderr_writer = utils::get_output_writer(redirections, RedirectionTarget::Stderr);

    match current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                writeln!(stdout_writer, "{}", path_str).unwrap();
            } else {
                writeln!(stderr_writer, "pwd: unable to convert path to string").unwrap();
            }
        }
        Err(e) => {
            writeln!(stderr_writer, "pwd: error retrieving current directory: {}", e).unwrap();
        }
    }
}

pub(crate) fn parse_pwd_cmd(args: &str, redirections: Vec<Redirection>) -> Result<Command, CommandError> {
    if args.trim().is_empty() {
        Ok(Command::Pwd { redirections })
    } else {
        Err(CommandError::InvalidArguments("pwd".to_string()))
    }
}