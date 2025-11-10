mod command;

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
        let builtins = vec!["echo", "exit", "type", "pwd", "cd"];
        let _line_up_to_cursor = &_line[.._pos];
        let start = _line_up_to_cursor
            .rfind(' ')
            .map_or(0, |idx| idx + 1);
        let current_word = &_line_up_to_cursor[start..];
        let candidates: Vec<String> = builtins
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