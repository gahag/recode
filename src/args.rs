use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// Supported conversion formats.
#[derive(Debug, Copy, Clone)]
pub enum InputFormat {
    Base64,
    Hex,
    Bin,
    Pem,
}

impl InputFormat {
    /// Whether whitespaces are data.
    pub fn whitespace_is_data(&self) -> bool {
        match self {
            Self::Base64 | Self::Hex => false,
            Self::Bin | Self::Pem => true,
        }
    }
}

impl FromStr for InputFormat {
    type Err = ParseFormatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            cmd if cmd.eq_ignore_ascii_case("hex") => Ok(Self::Hex),
            cmd if cmd.eq_ignore_ascii_case("base64") => Ok(Self::Base64),
            cmd if cmd.eq_ignore_ascii_case("bin") => Ok(Self::Bin),
            cmd if cmd.eq_ignore_ascii_case("pem") => Ok(Self::Pem),
            input => Err(ParseFormatError(input.to_owned())),
        }
    }
}

/// Supported conversion formats.
#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Base64,
    Hex,
    Bin,
    Pem(Pem),
}

/// Supported conversion formats.
#[derive(Debug, Copy, Clone)]
pub enum Pem {
    Crt,
    Csr,
    PrivateKey,
    EncryptedPrivateKey,
    PublicKey,
}

impl Pem {
    /// Get the PEM tag.
    pub fn tag(&self) -> &str {
        match self {
            Pem::Crt => "CERTIFICATE",
            Pem::Csr => "CERTIFICATE REQUEST",
            Pem::PrivateKey => "PRIVATE KEY",
            Pem::EncryptedPrivateKey => "ENCRYPTED PRIVATE KEY",
            Pem::PublicKey => "PUBLIC KEY",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = ParseFormatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            cmd if cmd.eq_ignore_ascii_case("hex") => Ok(Self::Hex),
            cmd if cmd.eq_ignore_ascii_case("base64") => Ok(Self::Base64),
            cmd if cmd.eq_ignore_ascii_case("bin") => Ok(Self::Bin),
            cmd if cmd.eq_ignore_ascii_case("crt") => Ok(Self::Pem(Pem::Crt)),
            cmd if cmd.eq_ignore_ascii_case("csr") => Ok(Self::Pem(Pem::Csr)),
            cmd if cmd.eq_ignore_ascii_case("key") => Ok(Self::Pem(Pem::PrivateKey)),
            cmd if cmd.eq_ignore_ascii_case("ekey") => Ok(Self::Pem(Pem::EncryptedPrivateKey)),
            cmd if cmd.eq_ignore_ascii_case("pub") => Ok(Self::Pem(Pem::PublicKey)),
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
    /// Input type: pem, base64, hex or bin.
    #[clap(short, long)]
    pub from: InputFormat,
    /// Output type: crt, csr, key, ekey, pub, base64, hex or bin.
    #[clap(short, long)]
    pub to: OutputFormat,
}
