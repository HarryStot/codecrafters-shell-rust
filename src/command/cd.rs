use std::path::PathBuf;
use std::env::var_os;
use std::env::set_current_dir;
use super::{Command, CommandError};

pub(crate) fn cd_cmd(args: &str) {
    let args = args.trim();
    let target = if args.is_empty() {
        // no argument -> HOME if set
        var_os("HOME").map(|h| PathBuf::from(h))
    } else if args.starts_with('~') {
        // expand ~ to HOME if possible, otherwise treat literally without the ~
        var_os("HOME").map(|h| {
            let rest = args.strip_prefix('~').unwrap_or("");
            PathBuf::from(h).join(rest)
        }).or_else(|| Some(PathBuf::from(args.strip_prefix('~').unwrap_or(args))))
    } else {
        Some(PathBuf::from(args))
    };

    match target {
        Some(path) => {
            if let Err(err) = set_current_dir(&path) {
                eprintln!("cd: {}: No such file or directory", path.display());
            }
        }
        None => eprintln!("cd: {}: HOME not set", args),
    }
}

pub(crate) fn parse_cd_cmd(args: &str) -> Result<Command, CommandError> {
    Ok(Command::Cd(args.to_owned()))
}