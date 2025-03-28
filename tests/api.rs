use anyhow::Result;
use assert_fs::prelude::*;

/// Ensure a single file "expands" correctly.
#[test]
fn expand_file() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("foo")?;
    file.touch()?;
    let expanded = sensei::expand_path(file.path(), 1, 1, 1)?;
    assert!(expanded.len() == 1);
    return Ok(());
}

/// Ensure a known directory with known files expands correctly
#[test]
fn expand_files() -> Result<()> {
    let dir = assert_fs::TempDir::new()?;
    dir.child("foo").touch()?;
    dir.child("bar").touch()?;

    let expanded = sensei::expand_path(dir.path(), 1, 2, 2)?;
    assert!(expanded.len() == 2);
    return Ok(());
}
