mod command;

use crate::command::Command;
use rustyline::config::Configurer;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Context, Editor, Helper};
use std::collections::HashSet;
use std::env::split_paths;
use std::sync::{Arc, RwLock};
use std::thread;

struct ShellHelper {
    executables_cache: Arc<RwLock<HashSet<String>>>,
}

impl ShellHelper {
    fn new(executables_cache: Arc<RwLock<HashSet<String>>>) -> Self {
        ShellHelper { executables_cache }
    }
}

impl Helper for ShellHelper {}
impl Hinter for ShellHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}
impl Validator for ShellHelper {}
impl Highlighter for ShellHelper {}

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = String::new();
    let first = &strings[0];
    for (i, ch) in first.char_indices() {
        for s in &strings[1..] {
            if s.get(i..).map_or(true, |sub| !sub.starts_with(ch)) {
                return prefix;
            }
        }
        prefix.push(ch);
    }
    prefix
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let start = line.rfind(' ').map_or(0, |idx| idx + 1);
        let current_word = &line[start..pos];

        // --- Gather Candidates from Cache ---
        let executables = self.executables_cache.read().unwrap();
        let mut candidates_set = executables.clone();
        
        let builtins = &["echo", "exit", "type", "pwd", "cd"];
        for builtin in builtins {
            candidates_set.insert(builtin.to_string());
        }

        let mut filtered_candidates: Vec<String> = candidates_set
            .into_iter()
            .filter(|cmd| cmd.starts_with(current_word))
            .collect();
        
        filtered_candidates.sort();

        // --- Handle results ---
        if filtered_candidates.is_empty() {
            Ok((start, Vec::new()))
        } else if filtered_candidates.len() == 1 {
            let candidate = filtered_candidates[0].clone() + " ";
            Ok((start, vec![candidate]))
        } else {
            let common_prefix = longest_common_prefix(&filtered_candidates);
            if !common_prefix.is_empty() && common_prefix.len() > current_word.len() {
                Ok((start, vec![common_prefix]))
            } else {
                Ok((start, filtered_candidates))
            }
        }
    }
}

fn main() {
    // --- Setup Cache in Background ---
    let executables_cache = Arc::new(RwLock::new(HashSet::new()));
    let cache_clone = Arc::clone(&executables_cache);
    thread::spawn(move || {
        let mut new_cache = HashSet::new();
        if let Ok(path_var) = std::env::var("PATH") {
            for path in split_paths(&path_var) {
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            if let Ok(metadata) = entry.metadata() {
                                if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                                    if let Some(file_name) = entry.file_name().to_str() {
                                        new_cache.insert(file_name.to_string());
                                    }
                                }
                            }
                        }
                        #[cfg(windows)]
                        {
                            if let Some(ext) = entry.path().extension() {
                                if ext == "exe" || ext == "bat" || ext == "cmd" {
                                    if let Some(file_name) = entry.file_name().to_str() {
                                        new_cache.insert(file_name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut cache = cache_clone.write().unwrap();
        *cache = new_cache;
    });

    // --- Setup Rustyline ---
    let mut rl = Editor::new().unwrap();
    let helper = ShellHelper::new(executables_cache);
    rl.set_helper(Some(helper));
    rl.set_completion_type(CompletionType::List);

    // --- Main Loop (starts immediately) ---
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