#![feature(drain_filter)]

mod args;

use std::io::{self, Read, Write};

use anyhow::Context;
use args::{Args, Command};
use clap::StructOpt;

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    let mut input = Vec::with_capacity(512);

    io::stdin()
        .lock()
        .read_to_end(&mut input)
        .context("Failed to read input")?;

    input
        .drain_filter(|c| c.is_ascii_whitespace())
        .for_each(|_| ());

    let decoded = match args.command {
        Command::Base64ToHex => base64::decode(input).context("Failed to decode base64")?,
        Command::HexToBase64 => hex::decode(input).context("Failed to decode hex")?,
    };

    let encoded = match args.command {
        Command::Base64ToHex => hex::encode(decoded),
        Command::HexToBase64 => base64::encode(decoded),
    };

    io::stdout().lock().write_all(encoded.as_bytes())?;

    Ok(())
}
