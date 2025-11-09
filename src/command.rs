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
        let input = input.trim().splitn(2, ' ').collect::<Vec<&str>>();
        let cmd = input.get(0).copied().unwrap_or("");
        let args_raw = input.get(1).copied().unwrap_or("");
        let args = utils::preprocess_args(args_raw);

        Ok(match cmd {
            "" => Noop,
            "echo" => echo::parse_echo_cmd(&args)?,
            "exit" => exit::parse_exit_cmd(&args)?,
            "type" => typee::parse_type_cmd(&args)?,
            "pwd" => pwd::parse_pwd_cmd(&args)?,
            "cd" => cd::parse_cd_cmd(&args)?,
            _ => match external::parse_external_cmd(cmd, &args) {
                Some(cmd) => cmd,
                None => return Err(CommandError::NotFound(cmd.to_string())),
            },
        })
    }

}
