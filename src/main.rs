use anyhow::{ensure, Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The file to look in
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    ensure!(!args.pattern.is_empty(), "no pattern specified");

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", args.path.display()))?;

    nconf::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    return Ok(());
}
