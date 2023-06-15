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
