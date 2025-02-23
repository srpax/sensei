use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use colored::Colorize; // for log messages
use log::{info, warn};
use std::collections::linked_list::LinkedList;
use std::fs;
use std::path::{Path, PathBuf};

struct ExpandArgs {
    depth: usize,
    max_depth: usize,
}

fn expand_path(dir: &Path, args: &ExpandArgs) -> Result<LinkedList<PathBuf>> {
    let mut expanded = LinkedList::new();
    if dir.is_dir() {
        // iterate through directory entries
        for result in fs::read_dir(dir)? {
            let dir = result?;
            let path = dir.path();
            if path.is_dir() {
                // path is to a directory
                if depth < MAX_DEPTH {
                    // recursively expand this directory
                    let opt_new_depth: Option<usize> = Some(depth + 1);
                    expanded.append(&mut expand_path(path.as_path(), opt_new_depth)?);
                } else {
                    // max recursion depth hit, warn and do nothing with this directory
                    warn!(
                        "{:width$}{}",
                        "",
                        path.to_str()
                            .context("Cannot display valid UTF-8 path")?
                            .strikethrough()
                            .bold()
                            .yellow(),
                        width = (depth * 2)
                    );
                }
            } else if path.is_file() {
                // path is to a file
                info!(
                    "{:width$}{}",
                    "",
                    path.to_str().unwrap_or("?").bold().green(),
                    width = (depth * 2)
                );
                expanded.push_back(path);
            } else {
                // path is not to a directory or file?
                warn!(
                    "{:width$}{}",
                    "",
                    path.to_str().unwrap_or("?").bold().red(),
                    width = (depth * 2)
                );
            }
        }
    }
    return Ok(expanded);
}

/// Expand a vector of paths to a linked list of file paths
/// TODO: add rules
pub fn expand_sources(
    source: &Vec<std::path::PathBuf>,
    args: &Args,
) -> Result<LinkedList<std::path::PathBuf>> {
    let mut expanded = LinkedList::new();
    for path in source {
        expanded.append(&mut expand_path(path.as_path(), None)?);
    }
    return Ok(expanded);
}
