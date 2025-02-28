use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag;
use log::info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// One or more source files or directories.
    sources: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Output: {}", args.output.display());

    info!("Input Sources:");
    for path in &args.sources {
        info!("  {}", path.display());
    }

    info!("Expanded Sources:");
    let expanded_paths = ningen::expand_paths(&args.sources, args.maxdepth)?;
    for expanded_path in expanded_paths {
        info!("  {}", expanded_path.display());
    }

    return Ok(());
}
