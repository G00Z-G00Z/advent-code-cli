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
    value_to_list_commands(
        target_lan_mapping.get(&Value::String("commands".into())),
        &mut template.commands,
    )?;

    // Init commands
    value_to_list_commands(
        target_lan_mapping.get(&Value::String("init_commands".into())),
        &mut template.init_commands,
    )?;

    // Folders
    if let Some(folders) = target_lan_mapping.get(&Value::String("folders".into())) {
        template.folders = folders
            .as_sequence()
            .ok_or_else(|| {
                YamlParserError::BadFormat("folders must be in a list. Check your format".into())
            })?
            .iter()
            .map(|command| {
                let command_str = command.as_str().ok_or_else(|| {
                    YamlParserError::BadFormat(
                        "Each init command must be a single string. Check your format".into(),
                    )
                })?;

                Ok(command_str.to_string())
            })
            .collect::<Result<Vec<String>, YamlParserError>>()?;
    }

    // Files
    if let Some(files) = target_lan_mapping.get(&Value::String("files".into())) {
        template.files = files
            .as_sequence()
            .ok_or_else(|| {
                YamlParserError::BadFormat("files must be in a list. Check your format".into())
            })?
            .iter()
            .map(|file| File::try_from(file))
            .collect::<Result<Vec<File>, YamlParserError>>()?;
    }

    Ok(template)
}

/// Takes a yaml value and returns a vector of commands
/// If the commands is not according to the tempalte, it will return an error
fn value_to_list_commands(
    values: Option<&Value>,
    template_command_vec: &mut Vec<Command>,
) -> Result<(), YamlParserError> {
    if let None = values {
        return Ok(());
    }

    let commands_sequence = values.unwrap().as_sequence().ok_or_else(|| {
        YamlParserError::BadFormat("commands must be in a list. Check your format".into())
    })?;

    *template_command_vec = commands_sequence
        .iter()
        .map(|command| Command::try_from(command))
        .collect::<Result<Vec<Command>, YamlParserError>>()?;
    Ok(())
}

impl TryFrom<&Value> for File {
    type Error = YamlParserError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(file_str) = value.as_str() {
            return Ok(File::empty(file_str));
        }

        if let Some(file_map) = value.as_mapping() {
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

                return Ok(File::new(name, Some(content_str.into())));
            } else {
                return Ok(File::empty(name));
            }
        }

        Err(YamlParserError::BadFormat(
            "Each file must be a string or a map. Check your format".into(),
        ))
    }
}

impl TryFrom<&Value> for Command {
    type Error = YamlParserError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let command_str = value.as_str().ok_or_else(|| {
            YamlParserError::BadFormat(
                "Each init command must be a single string. Check your format".into(),
            )
        })?;

        Ok(Command::new(command_str, vec![]))
    }
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

    #[test]
    fn test_error_when_bad_formating() {
        let file_content = "";
        let err = parse_values_yml(&file_content, "rust").unwrap_err();
        assert!(
            matches!(err, YamlParserError::NoLanguagesProvided),
            "Expected error when file is empty. Got {:?}",
            err,
        );

        let file_content = "
cosa:
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
        let err = parse_values_yml(&file_content, "rust").unwrap_err();

        assert!(
            matches!(
                parse_values_yml(&file_content, "rust").unwrap_err(),
                YamlParserError::NoLanguageFound(_)
            ),
            "Expected not to find language. Got {:?}",
            err,
        );

        let file_content = "
rust:
  commands:
    - cargo new {{title}}-{{year}}-{{day}}-{{language}}
       
  files:
    - mi: .env
      content: |
         DEMO_APP=1
    - src/lib.rs
    - input.txt
    - demo-input.txt
    - notes-day-{{day}}.txt

  folders: 
    - docs/
";

        assert!(
            matches!(
                parse_values_yml(&file_content, "rust").unwrap_err(),
                YamlParserError::BadFormat(_)
            ),
            "Expected error with files . Got {:?}",
            err,
        );
    }
}
