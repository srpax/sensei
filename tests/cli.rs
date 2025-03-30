use anyhow::Result;
use assert_fs::{prelude::FileTouch, NamedTempFile};

/// Ensure no source is accepted (in tty, would wait for input)
#[test]
fn source_none() -> Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.assert().success();
    return Ok(());
}

/// Ensure a single source via CLI is accepted.
#[test]
fn source_cli_single() -> Result<()> {
    let path = "this/is/test.c";

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg(path);

    cmd.assert()
        .success()
        .stdout(format!("build test: cc {path}\n"));

    return Ok(());
}

// Ensure a single source via stdin is accepted.
#[test]
fn source_stdin_single() -> Result<()> {
    let path = "this/is/test.c";

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.write_stdin(path.as_bytes());

    cmd.assert()
        .success()
        .stdout(format!("build test: cc {path}\n"));

    return Ok(());
}

/// Ensure multiple source files via CLI are accepted.
#[test]
fn source_cli_multiple() -> Result<()> {
    let path1 = "this/is/test1.c";
    let path2 = "this/is/test2.c";

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg(path1).arg(path2);

    cmd.assert().success().stdout(format!(
        "build test1: cc {path1}\nbuild test2: cc {path2}\n"
    ));

    return Ok(());
}

/// Ensure multiple source files via stdin are accepted.
#[test]
fn source_stdin_multiple() -> Result<()> {
    let path1 = "this/is/test1.c";
    let path2 = "this/is/test2.c";

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.write_stdin([path1, path2].join("\n").as_bytes());

    cmd.assert().success().stdout(format!(
        "build test1: cc {path1}\nbuild test2: cc {path2}\n"
    ));

    return Ok(());
}

#[test]
fn source_both_multiple() -> Result<()> {
    let path1 = "this/is/test1.c";
    let path2 = "this/is/test2.c";

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg(path1).write_stdin(path2.as_bytes());
    cmd.assert().success().stdout(format!(
        "build test1: cc {path1}\nbuild test2: cc {path2}\n"
    ));

    return Ok(());
}

#[test]
fn option_rule() -> Result<()> {
    let path = "this/is/test.c";
    let rule = "ld";

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg("--rule").arg(rule).arg(path);
    cmd.assert()
        .success()
        .stdout(format!("build test: {rule} {path}\n"));

    return Ok(());
}

#[test]
fn option_master_target() -> Result<()> {
    let path = "this/is/test.c";

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg("--master-target").arg("main").arg(path);
    cmd.assert()
        .success()
        .stdout(format!("build test: cc {path}\nbuild main: cc test\n"));

    return Ok(());
}

#[test]
fn option_master_rule() -> Result<()> {
    let path = "this/is/test.c";
    let mtarget = "main";
    let mrule = "ld";

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg("--master-target")
        .arg(mtarget)
        .arg("--master-rule")
        .arg(mrule)
        .arg(path);
    cmd.assert().success().stdout(format!(
        "build test: cc {path}\nbuild {mtarget}: {mrule} test\n"
    ));

    return Ok(());
}

#[test]
fn option_output_file() -> Result<()> {
    let path = "this/is/test.c";
    let output_file = NamedTempFile::new("targets.ninja")?;

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg("--output").arg(output_file.path()).arg(path);

    cmd.assert().success().stdout("");

    let target = std::fs::read_to_string(output_file.path())?;
    let target = target
        .split("\n")
        .nth(1)
        .expect("file should have at least 2 lines (banner + target)");
    assert_eq!(target, format!("build test: cc {path}"));

    return Ok(());
}

#[test]
fn error_output_exists() -> Result<()> {
    let path = "this/is/test.c";
    let output_file = NamedTempFile::new("targets.ninja")?;
    output_file.touch()?;

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg("--output").arg(output_file.path()).arg(path);

    cmd.assert().failure();

    return Ok(());
}

#[test]
fn option_force() -> Result<()> {
    let path = "this/is/test.c";
    let output_file = NamedTempFile::new("targets.ninja")?;
    output_file.touch()?;

    let mut cmd = assert_cmd::Command::cargo_bin("sensei")?;
    cmd.arg("--output")
        .arg(output_file.path())
        .arg("--force")
        .arg(path);

    cmd.assert().success();

    return Ok(());
}
