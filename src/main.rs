use std::io::{self, Read, Write};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let mut input = Vec::with_capacity(512);

    io::stdin().lock().read_to_end(&mut input)
        .context("Failed to read input")?;

    let decoded = base64::decode(input).context("Failed to decode base64")?;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for byte in decoded {
        write!(stdout, "{:x}", byte).context("Failed to write output")?;
    }

    Ok(())
}
