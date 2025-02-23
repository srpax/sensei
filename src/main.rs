use anyhow::Result;
use env_logger;
use log::info;

// Command Line Arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Append output to an existing output file, if it exists.
    #[arg(short, long)]
    append: bool,

    /// Load arguments from file.
    #[arg(short, long, exclusive = true)]
    file: bool,

    /// Set the path for the output ninja file.
    #[arg(short, long, default_value = "./build.ninja")]
    output: std::path::PathBuf,

    /// Recursively move through the source.
    #[arg(short, long)]
    recurse: bool,

    /// Set the maximum depth when recursively scanning source directories
    #[arg(long, default_value = "2")]
    maxdepth: usize,

    /// Surpress confirmation dialogs (e.g. when providing multiple source directories)
    #[arg(short, long)]
    surpress: bool,

    /// nipples
    #[command(flatten)]
    verbose: Verbosity,

    /// One or more source files or directories.
    source: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    info!("Sources:");
    for path in args.source {
        info!("  {}", path.display());
    }
    info!("Output: {:?}", args.output);
    return Ok(());
}
