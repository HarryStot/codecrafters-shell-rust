use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use super::{Redirection, RedirectionMode, RedirectionTarget};

pub fn open_file_for_redirection(redir: &Redirection) -> std::io::Result<File> {
    match redir.mode {
        RedirectionMode::Overwrite => File::create(&redir.file),
        RedirectionMode::Append => OpenOptions::new().create(true).append(true).open(&redir.file),
    }
}

// This function returns a "Writer" that is either a file or stdout/stderr.
pub fn get_output_writer(
    redirections: &[Redirection],
    target: RedirectionTarget,
) -> Box<dyn Write> {
    if let Some(redir) = redirections.iter().find(|r| r.target == target) {
        let file = open_file_for_redirection(redir)
            .expect("Failed to open redirection file");
        return Box::new(file);
    }

    // If no redirection is found for the target, return the standard stream.
    match target {
        RedirectionTarget::Stdout => Box::new(io::stdout()),
        RedirectionTarget::Stderr => Box::new(io::stderr()),
    }
}

pub(crate) fn preprocess_args(args: &str) -> String {
    let tokens = split_args(args);
    if tokens.is_empty() {
        String::new()
    } else {
        tokens.join(" ")
    }
}

pub(crate) fn split_args(raw: &str) -> Vec<String> {
    use std::iter::Peekable;
    use std::str::Chars;

    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    let mut iter: Peekable<Chars<'_>> = raw.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            '\\' => {
                if in_single_quotes {
                    // In single quotes backslash is literal
                    current.push('\\');
                } else if in_double_quotes {
                    // In double quotes, backslash only escapes a few chars: " \\ $ `
                    match iter.peek() {
                        Some(&next) => match next {
                            '"' | '\\' | '$' | '`' => {
                                // consume and push the escaped char
                                current.push(iter.next().unwrap());
                            }
                            '\n' => {
                                // backslash-newline: remove both (line continuation)
                                iter.next();
                            }
                            _ => {
                                // leave backslash as literal
                                current.push('\\');
                            }
                        },
                        None => {
                            // trailing backslash -> literal
                            current.push('\\');
                        }
                    }
                } else {
                    // Outside any quotes: backslash escapes next character (if any)
                    match iter.next() {
                        Some(next) => current.push(next),
                        None => current.push('\\'),
                    }
                }
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
                // else skip multiple spaces
            }
            other => current.push(other),
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}
