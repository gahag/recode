use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Base64ToHex,
    HexToBase64,
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            cmd if cmd.eq_ignore_ascii_case("hex") => Ok(Self::Base64ToHex),
            cmd if cmd.eq_ignore_ascii_case("base64") => Ok(Self::HexToBase64),
            input => Err(ParseCommandError(input.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct ParseCommandError(String);

impl Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid target: {}", self.0)
    }
}

impl std::error::Error for ParseCommandError {}

#[derive(Debug, Clone, clap::Parser)]
#[clap(version, author, about)]
pub struct Args {
    #[clap(short = 't')]
    pub command: Command,
}
