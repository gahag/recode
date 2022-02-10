use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// Supported conversion formats.
#[derive(Debug, Copy, Clone)]
pub enum Format {
    Base64,
    Hex,
    Bin,
}

impl Format {
    /// Whether the format may contain whitespace.
    pub fn allows_whitespace(&self) -> bool {
        match self {
            Self::Base64 | Self::Hex => false,
            Self::Bin => true,
        }
    }
}

impl FromStr for Format {
    type Err = ParseFormatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            cmd if cmd.eq_ignore_ascii_case("hex") => Ok(Self::Hex),
            cmd if cmd.eq_ignore_ascii_case("base64") => Ok(Self::Base64),
            cmd if cmd.eq_ignore_ascii_case("bin") => Ok(Self::Bin),
            input => Err(ParseFormatError(input.to_owned())),
        }
    }
}

/// Error while parsing format type.
#[derive(Debug)]
pub struct ParseFormatError(String);

impl Display for ParseFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid format: {}", self.0)
    }
}

impl std::error::Error for ParseFormatError {}

#[derive(Debug, Clone, clap::Parser)]
#[clap(version, author, about)]
pub struct Args {
    /// Input type: base64, hex or bin.
    #[clap(short, long)]
    pub from: Format,
    /// Output type: base64, hex or bin.
    #[clap(short, long)]
    pub to: Format,
}
