#![feature(drain_filter)]

mod args;

use std::io::{self, Read, Write};

use anyhow::Context;
use args::{Args, Format};
use clap::StructOpt;

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    let mut input = Vec::with_capacity(512);

    io::stdin()
        .lock()
        .read_to_end(&mut input)
        .context("Failed to read input")?;

    if !args.from.allows_whitespace() {
        input
            .drain_filter(|c| c.is_ascii_whitespace())
            .for_each(|_| ());
    }

    let decoded = match args.from {
        Format::Base64 => base64::decode(input).context("Failed to decode base64")?,
        Format::Hex => hex::decode(input).context("Failed to decode hex")?,
        Format::Bin => input,
    };

    let encoded = match args.to {
        Format::Base64 => base64::encode(decoded).into_bytes(),
        Format::Hex => hex::encode(decoded).into_bytes(),
        Format::Bin => decoded,
    };

    io::stdout().lock().write_all(&encoded)?;

    Ok(())
}
