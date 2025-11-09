use super::Command::{self, *};
use std::{io, io::Write, process::Command as StdCommand};

pub(crate) fn external_cmd(cmd: &Command) {
    let External { args, path, .. } = cmd else {
        eprintln!("Unexpected error occurred while executing external command");
        return;
    };
    let output = StdCommand::new(path)
        .args(args.iter())
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub(crate) fn parse_external_cmd(cmd: &str, args: &str) -> Option<Command> {
    let path_env = std::env::var_os("PATH")?;
    std::env::split_paths(&path_env)
        .map(|p| p.join(cmd))
        .find(|full_path| full_path.metadata().is_ok())
        .map(|path_buf| External {
            cmd: cmd.to_string(),
            args: args.split_whitespace().map(String::from).collect(),
            path: path_buf.to_string_lossy().to_string(),
        })
}
