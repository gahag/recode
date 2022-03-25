#![feature(drain_filter)]

mod args;

use std::io::{self, Read, Write};

use anyhow::Context;
use args::{Args, InputFormat, OutputFormat};
use clap::StructOpt;
use pem::Pem;

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    let mut input = Vec::with_capacity(512);

    io::stdin()
        .lock()
        .read_to_end(&mut input)
        .context("Failed to read input")?;

    if !args.from.whitespace_is_data() {
        input.drain_filter(|c| c.is_ascii_whitespace());
    }

    let decoded = match args.from {
        InputFormat::Pem => pem::parse(input).context("Failed to decode PEM")?.contents,
        InputFormat::Base64 => base64::decode(input).context("Failed to decode base64")?,
        InputFormat::Base64UrlSafe => {
            base64::decode_config(input, base64::URL_SAFE).context("Failed to decode base64")?
        }
        InputFormat::Hex => hex::decode(input).context("Failed to decode hex")?,
        InputFormat::Bin => input,
    };

    let encoded = match args.to {
        OutputFormat::Base64 => base64::encode(decoded).into_bytes(),
        OutputFormat::Base64UrlSafe => {
            base64::encode_config(decoded, base64::URL_SAFE).into_bytes()
        }
        OutputFormat::Hex => hex::encode(decoded).into_bytes(),
        OutputFormat::Bin => decoded,
        OutputFormat::Pem(pem) => pem::encode_config(
            &Pem {
                tag: pem.tag().into(),
                contents: decoded,
            },
            pem::EncodeConfig {
                line_ending: pem::LineEnding::LF,
            },
        )
        .into_bytes(),
    };

    io::stdout().lock().write_all(&encoded)?;

    Ok(())
}
