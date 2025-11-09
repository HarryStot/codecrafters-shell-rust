use super::{Command, CommandError};

pub(crate) fn type_cmd(cmd: &String) {
    use Command::*;
    if cmd.is_empty() {
        eprint!("");
        return;
    }
    match Command::from(cmd) {
        Ok(External { path, .. }) => {
            println!("{} is {}", cmd, path);
        }
        Ok(_) => {
            println!("{} is a shell builtin", cmd);
        }
        Err(CommandError::NotFound(..)) => {
            eprintln!("{}: not found", cmd)
        }
        Err(e) => {
            eprintln!("type: error: {}", e);
        }
    };
}

pub(crate) fn parse_type_cmd(args: &str) -> Result<Command, CommandError> {
    let cmd = args.trim().to_owned();
    Ok(Command::Type(cmd))
}