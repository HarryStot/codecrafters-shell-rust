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

pub(crate) fn split_preprocessed_args(preprocessed: &str) -> Vec<String> {
    if preprocessed.is_empty() {
        Vec::new()
    } else {
        preprocessed.split(' ').map(String::from).collect()
    }
}

