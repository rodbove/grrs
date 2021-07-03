use anyhow::{Context, Result};

pub fn find_matches(
  content: &str,
  pattern: &str,
  mut writter: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
  for line in content.lines() {
      if line.contains(&pattern) {
          writeln!(writter, "{}", line)
              .with_context(|| format!("Could not read line: `{:?}`", &line))?;
      }
  }

  Ok(())
}

#[test]
fn find_a_match() -> Result<(), Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    find_matches(
        "lorem ipsum\ntest ipsum\nnothing ipsum",
        "lorem",
        &mut result,
    )?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}
