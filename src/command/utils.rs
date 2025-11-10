// Shared utilities for command argument preprocessing and splitting

pub(crate) fn preprocess_args(args: &str) -> String {
    let mut out = String::with_capacity(args.len());
    let mut in_quote = false;
    let mut last_was_space = false;

    for c in args.chars() {
        match c {
            '\'' => in_quote = !in_quote,
            ' ' => {
                if in_quote {
                    out.push(' ');
                    last_was_space = false;
                } else if !last_was_space {
                    out.push(' ');
                    last_was_space = true;
                }
            }
            _ => {
                out.push(c);
                last_was_space = false;
            }
        }
    }

    if last_was_space {
        out.pop();
    }

    out
}

pub(crate) fn split_args_respecting_single_quotes(raw: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in raw.chars() {
        match c {
            '\'' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
                // if current is empty, skip extra separators
            }
            ch => current.push(ch),
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}
