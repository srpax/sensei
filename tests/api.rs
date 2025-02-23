use anyhow::Result;
use assert_fs::prelude::*;

/// Ensure a known directory with known files expands correctly
#[test]
fn expand_directory() -> Result<()> {
    let mut paths = Vec::new();
    let dir = assert_fs::TempDir::new()?;

    let file1 = dir.child("foo");
    let file2 = dir.child("bar");
    file1.touch()?;
    file2.touch()?;

    paths.push(dir.to_path_buf());
    let paths = ningen::expand_sources(&paths)?;

    assert!(paths.len() == 2);
    return Ok(());
}
