pub(crate) fn preprocess_args(args: &str) -> String {
    let tokens = split_args(args);
    if tokens.is_empty() {
        String::new()
    } else {
        tokens.join(" ")
    }
}

pub(crate) fn split_args(raw: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut last_was_backslash = false;

    for c in raw.chars() {
        if last_was_backslash {
            current.push(c);
            last_was_backslash = false;
            continue;
        }

        match c {
            '\\' => {
                last_was_backslash = true;
            }
            '"' => {
                if in_single_quotes {
                    current.push(c);
                } else {
                    in_double_quotes = !in_double_quotes;
                }
            }
            '\'' => {
                if in_double_quotes {
                    current.push(c);
                } else {
                    in_single_quotes = !in_single_quotes;
                }
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
                // if current is empty, skip extra separators
            }
            ch => current.push(ch),
        }
    }

    if last_was_backslash {
        current.push('\\');
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}
