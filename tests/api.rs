use anyhow::Result;
use assert_fs::prelude::*;

#[test]
fn expand_source() -> Result<()> {
    let mut paths = Vec::new();
    let file = assert_fs::NamedTempFile::new("foo")?;

    paths.push(file.path());

    for path in paths {}
    let expanded_paths = ningen::expand_paths(&paths, 2);
    return Ok(());
}
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

    let mut args = ningen::PathExpander::new(2);
    let paths = ningen::expand_sources(&paths, &mut args)?;

    assert!(paths.len() == 2);
    return Ok(());
}
