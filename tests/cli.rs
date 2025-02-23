use assert_cmd::Command; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions

const APP_NAME: &str = "ningen";

/// ensure a nonexistant path is detected
#[test]
fn input_source_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::PathBuf::from("this/is/not/a/path");
    let mut cmd = Command::cargo_bin(APP_NAME)?;
    assert!(!path.exists());
    cmd.arg(path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
    return Ok(());
}

/// ensure a single source file is detected
#[test]
fn input_single_source_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("foo")?;
    let mut cmd = Command::cargo_bin(APP_NAME)?;

    // flush the file
    file.write_str("\n")?; // flush?
    cmd.arg(file.path());
    cmd.assert().success();

    return Ok(());
}

/// Ensure multiple source files are detected
#[test]
fn input_multiple_source_file() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = assert_fs::NamedTempFile::new("foo")?;
    let file2 = assert_fs::NamedTempFile::new("bar")?;
    let mut cmd = Command::cargo_bin(APP_NAME)?;

    // flush files
    file1.write_str("\n")?;
    file2.write_str("\n")?;

    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();

    return Ok(());
}

/// Ensure a single directory is detected correctly
#[test]
fn input_single_source_dir() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new()?;
    let mut cmd = Command::cargo_bin(APP_NAME)?;
    cmd.arg(dir.path());
    cmd.assert().success();
    return Ok(());
}

/// Ensure multiple directories are detected correctly
#[test]
fn input_multiple_source_dir() -> Result<(), Box<dyn std::error::Error>> {
    let dir1 = assert_fs::TempDir::new()?;
    let dir2 = assert_fs::TempDir::new()?;
    let mut cmd = Command::cargo_bin(APP_NAME)?;
    cmd.arg(dir1.path()).arg(dir2.path());
    cmd.assert().success();
    return Ok(());
}

#[test]
fn input_various_sources() -> Result<(), Box<dyn std::error::Error>> {
    let dir1 = assert_fs::TempDir::new()?;
    let dir2 = assert_fs::TempDir::new()?;
    let file1 = dir1.child("foo");
    let file2 = dir2.child("bar");

    let mut cmd = Command::cargo_bin(APP_NAME)?;

    // flush files
    file1.write_str("\n")?;
    file2.write_str("\n")?;

    cmd.arg(file1.path())
        .arg(file2.path())
        .arg(dir1.path())
        .arg(dir2.path());
    cmd.assert().success();

    return Ok(());
}
