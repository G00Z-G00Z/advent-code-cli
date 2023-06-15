use std::{collections::HashMap, fs};
use tera::Tera;

use crate::interfaces::DayChallenge;

/// Error type for the yaml parser
#[derive(Debug)]
pub enum YamlParserError {
    TeraError(tera::Error),
}

/// Parses the yml file with the given metadata
pub fn parse_yml_file(
    yml_file_content: &str,
    day_challenge: &DayChallenge,
) -> Result<String, YamlParserError> {
    let mut tera = Tera::default();

    let mut context = tera::Context::new();

    // Project metadata
    let user_provided_vars = [
        ("day", &day_challenge.day.to_string()),
        ("year", &day_challenge.year.to_string()),
        ("language", &day_challenge.language.to_string()),
        ("title", &day_challenge.title.to_string()),
    ];

    // Insert the metadata into the context
    for (key, value) in user_provided_vars.iter() {
        context.insert(*key, *value);
    }

    let rendered = tera
        .render_str(&yml_file_content, &context)
        .map_err(|e| YamlParserError::TeraError(e))?;

    Ok(rendered)
}

#[cfg(test)]
mod tests {

    use std::fmt::format;

    use super::*;

    #[test]
    fn test_parse_yml_file_without_variables() {
        let metadata = DayChallenge::new(1, 2020, "Rust".to_string(), "test-2-numbers".into());

        let file_content = "
rust:
  commands:
    - cargo new hello
       
  files:
    - name: .env
      content: |
         DEMO_APP=1
    - src/lib.rs
    - input.txt
    - demo-input.txt

  folders: 
    - docs/
";

        let expected = "
rust:
  commands:
    - cargo new hello
       
  files:
    - name: .env
      content: |
         DEMO_APP=1
    - src/lib.rs
    - input.txt
    - demo-input.txt

  folders: 
    - docs/
";

        let parsed_content = parse_yml_file(&file_content, &metadata).unwrap();

        assert_eq!(&parsed_content, &expected);
    }

    #[test]
    fn test_parse_yml_file_with_variables() {
        let metadata = DayChallenge::new(1, 2020, "Rust".to_string(), "test-2-numbers".into());

        let file_content = "
rust:
  commands:
    - cargo new {{title}}-{{year}}-{{day}}-{{language}}
       
  files:
    - name: .env
      content: |
         DEMO_APP=1
    - src/lib.rs
    - input.txt
    - demo-input.txt
    - notes-day-{{day}}.txt

  folders: 
    - docs/
";

        let expected = format!(
            "
rust:
  commands:
    - cargo new {}-{}-{}-{}
       
  files:
    - name: .env
      content: |
         DEMO_APP=1
    - src/lib.rs
    - input.txt
    - demo-input.txt
    - notes-day-{}.txt

  folders: 
    - docs/
",
            &metadata.title, &metadata.year, &metadata.day, &metadata.language, &metadata.day
        );

        let parsed_content = parse_yml_file(&file_content, &metadata).unwrap();

        assert_eq!(&parsed_content, &expected);
    }
}
