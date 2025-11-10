use error::CommandError;

mod cd;
pub(crate) mod echo;
pub(crate) mod error;
pub(crate) mod exit;
pub(crate) mod external;
pub(crate) mod pwd;
pub(crate) mod typee;
pub(crate) mod utils; // extracted shared utilities

// Represents whether to overwrite (>) or append (>>)
#[derive(Debug)]
pub enum RedirectionMode {
    Overwrite,
    Append,
}

// Represents the output stream to be redirected
#[derive(Debug, PartialEq)]
pub enum RedirectionTarget {
    Stdout,
    Stderr,
}

// A struct to hold all information about a single redirection
#[derive(Debug)]
pub struct Redirection {
    pub target: RedirectionTarget,
    pub file: String,
    pub mode: RedirectionMode,
}

#[derive(Debug)]
pub enum Command {
    Noop,
    Exit(i32),
    Cd(String),
    Echo {
        message: String,
        redirections: Vec<Redirection>,
    },
    Type {
        cmd: String,
        redirections: Vec<Redirection>,
    },
    Pwd {
        redirections: Vec<Redirection>,
    },
    External {
        cmd: String,
        args: Vec<String>,
        path: String,
        redirections: Vec<Redirection>,
    },
}

impl Command {
    pub fn execute(&self) {
        use Command::*;
        match self {
            Noop => (),
            Exit(code) => exit::exit_cmd(*code),
            Cd(path) => cd::cd_cmd(path),

            Echo {
                message,
                redirections,
            } => {
                let mut stdout_writer = utils::get_output_writer(redirections, RedirectionTarget::Stdout);
                // Eagerly get stderr writer to create/truncate the file, even if unused.
                let _stderr_writer = utils::get_output_writer(redirections, RedirectionTarget::Stderr);
                echo::echo_cmd(message, &mut stdout_writer);
            }
            Type { cmd, redirections } => {
                let mut stdout_writer = utils::get_output_writer(redirections, RedirectionTarget::Stdout);
                let mut stderr_writer = utils::get_output_writer(redirections, RedirectionTarget::Stderr);
                typee::type_cmd(cmd, &mut stdout_writer, &mut stderr_writer);
            }
            Pwd { redirections } => {
                let mut stdout_writer = utils::get_output_writer(redirections, RedirectionTarget::Stdout);
                let mut stderr_writer = utils::get_output_writer(redirections, RedirectionTarget::Stderr);
                pwd::pwd_cmd(&mut stdout_writer, &mut stderr_writer);
            }
            External { .. } => external::external_cmd(self),
        }
    }

    pub fn from(input: &str) -> Result<Command, CommandError> {
        use Command::*;
        let input = input.trim();
        let mut input_tokens = utils::split_args(input);

        // Use Options to ensure the last redirection for each stream is the one that's kept.
        let mut stdout_redir = None;
        let mut stderr_redir = None;

        let mut i = 0;
        while i < input_tokens.len() {
            let (target, mode) = match input_tokens[i].as_str() {
                ">" => (RedirectionTarget::Stdout, RedirectionMode::Overwrite),
                "1>" => (RedirectionTarget::Stdout, RedirectionMode::Overwrite),
                ">>" => (RedirectionTarget::Stdout, RedirectionMode::Append),
                "1>>" => (RedirectionTarget::Stdout, RedirectionMode::Append),
                "2>" => (RedirectionTarget::Stderr, RedirectionMode::Overwrite),
                "2>>" => (RedirectionTarget::Stderr, RedirectionMode::Append),
                _ => {
                    i += 1;
                    continue;
                }
            };

            if i + 1 >= input_tokens.len() {
                return Err(CommandError::InvalidArguments(
                    "Missing file for redirection".to_string(),
                ));
            }
            let file = input_tokens.remove(i + 1);
            input_tokens.remove(i);

            let redir = Redirection { target, file, mode };
            match redir.target {
                RedirectionTarget::Stdout => stdout_redir = Some(redir),
                RedirectionTarget::Stderr => stderr_redir = Some(redir),
            }
        }

        let mut redirections = Vec::new();
        if let Some(r) = stdout_redir {
            redirections.push(r);
        }
        if let Some(r) = stderr_redir {
            redirections.push(r);
        }

        let cmd = input_tokens.first().map(|s| s.as_str()).unwrap_or("");
        let args_tokens: Vec<String> = if input_tokens.len() > 1 {
            input_tokens[1..].to_vec()
        } else {
            Vec::new()
        };
        let args_for_builtins = if args_tokens.is_empty() {
            String::new()
        } else {
            args_tokens.join(" ")
        };

        Ok(match cmd {
            "" => Noop,
            "echo" => echo::parse_echo_cmd(&args_for_builtins, redirections)?,
            "exit" => exit::parse_exit_cmd(&args_for_builtins)?,
            "type" => typee::parse_type_cmd(&args_for_builtins, redirections)?,
            "pwd" => pwd::parse_pwd_cmd(&args_for_builtins, redirections)?,
            "cd" => cd::parse_cd_cmd(&args_for_builtins)?,
            _ => match external::parse_external_cmd(cmd, args_tokens, redirections) {
                Some(cmd) => cmd,
                None => return Err(CommandError::NotFound(cmd.to_string())),
            },
        })
    }
}
