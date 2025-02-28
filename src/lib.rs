use anyhow::{Context, Result};
use colored::Colorize; // for log messages
use log::{error, info, warn};
use std::collections::linked_list::LinkedList;
use std::fs;
use std::path::{Path, PathBuf};

/// Arguments specifically used for expanding source path.
pub struct PathExpander {
    /// The current recursive depth.
    pub depth: usize,

    /// The maximum recursion depth.
    max_depth: usize,
}

impl PathExpander {
    /// Create new arguments with the specified max recursion depth
    pub fn new(max_depth: usize) -> Self {
        Self {
            depth: 0,
            max_depth,
        }
    }

    // Expand a path recursively
    pub fn expand(&mut self, path: PathBuf) -> Result<LinkedList<PathBuf>> {
        let mut expanded_paths = LinkedList::new();
        let path_str = format!("{}", path.display());
        if self.depth > self.max_depth {
            // log ignored path
            warn!(
                "{:width$}{}",
                "",
                path_str.strikethrough().yellow(),
                width = (self.depth * 2)
            );
        } else if !path.exists() {
            // log missing path
            error!(
                "{:width$}{}",
                "",
                path_str.strikethrough().bold().red(),
                width = (self.depth * 2)
            )
        } else {
            // log existing path!
            info!(
                "{:width$}{}",
                "",
                path_str.green(),
                width = (self.depth * 2)
            );
            if path.is_file() {
                // path is a file
                expanded_paths.push_back(path);
            } else if path.is_dir() {
                // path is a directory
                self.depth += 1;
                for subpath in fs::read_dir(path)? {
                    // recursively expand this path
                    expanded_paths.append(&mut self.expand(subpath?.path())?);
                }
            } else {
                // path is unknown filesystem object
                warn!(
                    "{:width$}{}",
                    "",
                    path_str.strikethrough().italic().blue(),
                    width = (self.depth * 2)
                );
            }
        }
        return Ok(expanded_paths);
    }
}

/// Expand multiple paths as needed.
pub fn expand_paths(paths: &Vec<PathBuf>, max_depth: usize) -> Result<LinkedList<PathBuf>> {
    let mut expanded_paths = LinkedList::new();
    for path in paths {
        expanded_paths.append(&mut PathExpander::new(max_depth).expand(path.clone())?);
    }
    return Ok(expanded_paths);
}
