use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag;
use log::info;
use std::path::PathBuf;

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

    /// Maximum depth when recursively scanning source directories.
    #[arg(long, default_value = "2")]
    maxdepth: usize,

    /// Initial path buffer size when expanding paths.
    /// Reallocation may occur if actual expanded path count exceeds this number.
    /// Default is 256.
    #[arg(long, default_value = "256")]
    initpaths: usize,

    /// Maximum path buffer size when expanding paths.
    /// Operations will fail expanded path count exceeds this number.
    /// Default is 2048.
    #[arg(long, default_value = "2048")]
    maxpaths: usize,

    /// Surpress confirmation dialogs (e.g. when providing multiple source directories)
    #[arg(short, long)]
    surpress: bool,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// One or more source files or directories.
    #[arg()]
    sources: Vec<std::path::PathBuf>,
}

/// Expand multiple paths.
fn expand_paths(
    paths: &Vec<PathBuf>,
    max_depth: usize,
    init_count: usize,
    max_count: usize,
) -> Result<Vec<PathBuf>> {
    let mut expanded_paths: Vec<PathBuf> = Vec::new();
    for path in paths {
        expanded_paths.append(&mut sensei::expand_path(
            path.as_path(),
            max_depth,
            init_count,
            max_count,
        )?);
    }
    return Ok(expanded_paths);
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Output: {}", args.output.display());

    info!("Input:");
    for path in &args.sources {
        info!("  {}", path.display());
    }

    let exts: sensei::ExtMap = vec![(Some("c".into()), Some("o".into()))];
    let expanded_paths = expand_paths(&args.sources, args.maxdepth, args.initpaths, args.maxpaths)?;
    sensei::make_target_file(&args.output, &expanded_paths, &exts)?;

    return Ok(());
}
