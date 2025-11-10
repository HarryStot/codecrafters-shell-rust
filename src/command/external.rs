use super::Command::{self, *};
use std::{io, io::Write, process::Command as StdCommand};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;

pub(crate) fn external_cmd(cmd: &Command) {
    let External { cmd: cmd_name, args, path, .. } = cmd else {
        eprintln!("Unexpected error occurred while executing external command");
        return;
    };
    let output = StdCommand::new(path)
        .arg0(cmd_name)
        .args(args.iter())
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub(crate) fn parse_external_cmd(cmd: &str, raw_args: &str) -> Option<Command> {
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
            args: super::utils::split_args(raw_args),
            path: path_buf.to_string_lossy().to_string(),
        })
}
