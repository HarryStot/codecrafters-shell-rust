mod command;

use std::collections::HashSet;
use std::env::split_paths;
use crate::command::Command;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

struct ShellHelper;

impl Helper for ShellHelper {}

impl Hinter for ShellHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Validator for ShellHelper {}

impl Highlighter for ShellHelper {}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let start = _line.rfind(' ').map_or(0, |idx| idx + 1);
        let current_word = &_line[start.._pos];

        // --- Gather candidates ---
        let mut builtins = vec!["echo", "exit", "type", "pwd", "cd"];

        let mut candidates_set = HashSet::new();
        if let Ok(path_var) = std::env::var("PATH") {
            for path in split_paths(&path_var) {
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        if let Ok(metadata) = entry.metadata() {
                            #[cfg(unix)]
                            {
                                use std::os::unix::fs::PermissionsExt;
                                if metadata.permissions().mode() & 0o111 != 0 {
                                    if let Some(file_name) = entry.file_name().to_str() {
                                        candidates_set.insert(file_name.to_string());
                                    }
                                }
                            }
                            #[cfg(windows)]
                            {
                                if let Some(ext) = entry.path().extension() {
                                    if ext == "exe" || ext == "bat" || ext == "cmd" {
                                        if let Some(file_name) = entry.file_name().to_str() {
                                            candidates_set.insert(file_name.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {}

        for builtin in builtins { candidates_set.insert(builtin.to_string()); }

        let candidates: Vec<String> = candidates_set
            .into_iter()
            .filter(|cmd| cmd.starts_with(current_word))
            .map(|s| (s.to_owned() + " ").to_string())
            .collect();

        if !candidates.is_empty() {
            Ok((start, candidates))
        } else {
            Ok((0, Vec::new()))
        }
    }
}

fn main() {
    let mut rl = Editor::new().unwrap();
    rl.set_helper(Some(ShellHelper));

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                match Command::from(line.trim()) {
                    Ok(cmd) => cmd.execute(),
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C: new line, do nothing
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D: exit gracefully
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}