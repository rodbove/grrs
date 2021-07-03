use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::io::Write;
use std::process::Command; // Run programs
use tempfile::NamedTempFile; // Creates temp file

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("grrs")?;

  cmd.arg("foobar").arg("test/file/doesnt/exist");
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("No such file or directory"));

  Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
  let mut file = NamedTempFile::new()?;
  writeln!(file, "Temporary test file\nlorem ipsum\ntest ipsum\nloremus")?;

  let mut cmd = Command::cargo_bin("grrs")?;

  cmd.arg("lorem").arg(file.path());
  cmd.assert()
    .success()
    .stdout(predicate::str::contains("lorem ipsum\nloremus"));

  Ok(())
}

#[test]
fn find_empty_pattern() -> Result<(), Box<dyn std::error::Error>> {
  let mut file = NamedTempFile::new()?;
  writeln!(file, "Temporary test file\nlorem ipsum\ntest ipsum\nloremus")?;

  let mut cmd = Command::cargo_bin("grrs")?;

  cmd.arg("").arg(file.path());
  cmd.assert()
    .success()
    .stdout(predicate::str::contains("Temporary test file\nlorem ipsum\ntest ipsum\nloremus"));

  Ok(())
}