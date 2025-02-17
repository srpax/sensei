use anyhow::{Context, Result};

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| format!("could not write matches"))?;
        }
    }
    return Ok(());
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n");
}
