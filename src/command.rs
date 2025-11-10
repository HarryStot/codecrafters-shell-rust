use error::CommandError;

pub(crate) mod echo;
pub(crate) mod error;
pub(crate) mod exit;
pub(crate) mod external;
pub(crate) mod typee;
pub(crate) mod pwd;
mod cd;
pub(crate) mod utils; // extracted shared utilities

pub enum Command {
    Noop,
    Exit(i32),
    Echo(String),
    Type(String),
    Pwd,
    Cd(String),
    External {
        cmd: String,
        args: Vec<String>,
        path: String,
    }
}

impl Command {
    pub fn execute(&self) {
        use Command::*;
        match self {
            Noop => (),
            Exit(code) => exit::exit_cmd(*code),
            Echo(message) => echo::echo_cmd(message),
            Type(cmd) => typee::type_cmd(cmd),
            Pwd => pwd::pwd_cmd(),
            Cd(path) => cd::cd_cmd(path),
            External { .. } => external::external_cmd(self),
        }
    }

    pub fn from(input: &str) -> Result<Command, CommandError> {
        use Command::*;
        let input = input.trim();
        let input_tokens = utils::split_args(input);
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
            "echo" => echo::parse_echo_cmd(&args_for_builtins)?,
            "exit" => exit::parse_exit_cmd(&args_for_builtins)?,
            "type" => typee::parse_type_cmd(&args_for_builtins)?,
            "pwd" => pwd::parse_pwd_cmd(&args_for_builtins)?,
            "cd" => cd::parse_cd_cmd(&args_for_builtins)?,
            _ => match external::parse_external_cmd(cmd, args_tokens) {
                Some(cmd) => cmd,
                None => return Err(CommandError::NotFound(cmd.to_string())),
            },
        })
    }

}
