use super::Command::{self, *};
use super::{utils, Redirection, RedirectionTarget};
use std::{io, io::Write, process::{Command as StdCommand, Stdio}};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;

pub(crate) fn external_cmd(cmd: &Command) {
    let External { cmd: cmd_name, args, path, redirections } = cmd else {
        eprintln!("Unexpected error occurred while executing external command");
        return;
    };

    let mut command = StdCommand::new(path);
    command.arg0(cmd_name).args(args.iter());

    let mut stdout_redirected = false;
    let mut stderr_redirected = false;

    for redirection in redirections {
        let file = utils::open_file_for_redirection(redirection)
            .expect("Failed to open redirection file");

        let stdio = Stdio::from(file);

        match redirection.target {
            RedirectionTarget::Stdout => {
                command.stdout(stdio);
                stdout_redirected = true;
            }
            RedirectionTarget::Stderr => {
                command.stderr(stdio);
                stderr_redirected = true;
            }
        }
    }

    // Fallback to piped output if not redirected, so you can print to console
    if !stdout_redirected { command.stdout(Stdio::piped()); }
    if !stderr_redirected { command.stderr(Stdio::piped()); }


    let output = command.output().expect("failed to execute process");

    if !stdout_redirected {
        io::stdout().write_all(&output.stdout).unwrap();
    }
    if !stderr_redirected {
        io::stderr().write_all(&output.stderr).unwrap();
    }
}

pub(crate) fn parse_external_cmd(cmd: &str, args_tokens: Vec<String>, redirections: Vec<Redirection>) -> Option<Command> {
    let path_env = std::env::var_os("PATH")?;
    std::env::split_paths(&path_env)
        .map(|p| p.join(cmd))
        .find(|full_path| {
            full_path.metadata()
                .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
        .map(|path_buf| External {
            cmd: cmd.to_string(),
            args: args_tokens,
            path: path_buf.to_string_lossy().to_string(),
            redirections,
        })
}
