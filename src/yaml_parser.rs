use serde_yaml::{from_str, Value};
use tera::Tera;

use crate::interfaces::*;

/// Error type for the yaml parser
#[derive(Debug)]
pub enum YamlParserError {
    TeraError(tera::Error),
    YamlError(serde_yaml::Error),
    NoLanguagesProvided,
    NoLanguageFound(String),
    BadFormat(String),
}

/// Parses the yml file with the given metadata
pub fn populate_yml(
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

pub fn parse_values_yml(
    yaml_content: &str,
    language: &str,
) -> Result<ProgrammingTemplate, YamlParserError> {
    let parsed_value: Value = from_str(yaml_content).map_err(YamlParserError::YamlError)?;

    let mut template = ProgrammingTemplate::default();

    let available_langs = parsed_value
        .as_mapping()
        .ok_or_else(|| YamlParserError::NoLanguagesProvided)?;

    let target_lan_mapping = available_langs
        .get(&Value::String(language.to_string()))
        .ok_or(YamlParserError::NoLanguageFound(language.to_string()))?;

    template.language = language.into();

    // Init commands
    if let Some(init_commands) = target_lan_mapping.get(&Value::String("init_commands".into())) {
        let init_commands_sequence = init_commands.as_sequence().ok_or_else(|| {
            YamlParserError::BadFormat("init_commands must be in a list. Check your format".into())
        })?;

        for command in init_commands_sequence {
            let command_str = command.as_str().ok_or_else(|| {
                YamlParserError::BadFormat(
                    "Each init command must be a single string. Check your format".into(),
                )
            })?;

            template
                .init_commands
                .push(Command::new(command_str, vec![]));
        }
    }

    // commands
    if let Some(commands) = target_lan_mapping.get(&Value::String("commands".into())) {
        let commands_sequence = commands.as_sequence().ok_or_else(|| {
            YamlParserError::BadFormat("commands must be in a list. Check your format".into())
        })?;

        for command in commands_sequence {
            let command_str = command.as_str().ok_or_else(|| {
                YamlParserError::BadFormat(
                    "Each init command must be a single string. Check your format".into(),
                )
            })?;

            template.commands.push(Command::new(command_str, vec![]));
        }
    }

    // Folders
    if let Some(folders) = target_lan_mapping.get(&Value::String("folders".into())) {
        let folders_sequence = folders.as_sequence().ok_or_else(|| {
            YamlParserError::BadFormat("folders must be in a list. Check your format".into())
        })?;

        for command in folders_sequence {
            let command_str = command.as_str().ok_or_else(|| {
                YamlParserError::BadFormat(
                    "Each init command must be a single string. Check your format".into(),
                )
            })?;

            template.folders.push(command_str.to_string());
        }
    }

    // Files
    if let Some(files) = target_lan_mapping.get(&Value::String("files".into())) {
        let files_sequence = files.as_sequence().ok_or_else(|| {
            YamlParserError::BadFormat("files must be in a list. Check your format".into())
        })?;

        for file in files_sequence {
            if let Some(file_str) = file.as_str() {
                template.files.push(File::empty(file_str));
                continue;
            }

            if let Some(file_map) = file.as_mapping() {
                let name = file_map
                    .get(&Value::String("name".into()))
                    .ok_or_else(|| {
                        YamlParserError::BadFormat(
                            "Each file must have a name. Check your format".into(),
                        )
                    })?
                    .as_str()
                    .ok_or_else(|| {
                        YamlParserError::BadFormat(
                            "Each file name must be a single string. Check your format".into(),
                        )
                    })?;

                let content = file_map.get(&Value::String("content".into()));

                if let Some(content) = content {
                    let content_str = content.as_str().ok_or_else(|| {
                        YamlParserError::BadFormat(
                            "Each file content must be a single string. Check your format".into(),
                        )
                    })?;

                    template
                        .files
                        .push(File::new(name, Some(content_str.into())));
                } else {
                    template.files.push(File::empty(name));
                }
                continue;
            }

            return Err(YamlParserError::BadFormat(
                "Each file must be a string or a map. Check your format".into(),
            ));
        }
    }

    Ok(template)
}

#[cfg(test)]
mod tests {

    use crate::interfaces::{Command, File};

    use super::*;

    #[test]
    fn test_parse_yml_values() {
        let file_content = "
rust:

  init_commands:
    - cargo new hello

  commands:
    - cargo add serde
       
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

        let template = parse_values_yml(&file_content, "rust").unwrap();

        let expected = ProgrammingTemplate {
            language: "rust".to_string(),
            init_commands: vec![Command::new("cargo new hello", vec![])],
            commands: vec![Command::new("cargo add serde", vec![])],
            files: vec![
                File::new(".env", Some("DEMO_APP=1\n".to_string())),
                File::empty("src/lib.rs"),
                File::empty("input.txt"),
                File::empty("demo-input.txt"),
            ],
            folders: vec!["docs/".to_string()],
        };

        assert_eq!(template, expected, "Expected template to be equal");
    }

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

        let parsed_content = populate_yml(&file_content, &metadata).unwrap();

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

        let parsed_content = populate_yml(&file_content, &metadata).unwrap();

        assert_eq!(&parsed_content, &expected);
    }
}
