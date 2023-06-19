use std::fmt::Display;

/// Metadata of a Day Challege
#[derive(Debug)]
pub struct DayChallenge {
    /// Number of the day
    pub day: u8,
    /// Year of the day
    pub year: u16,
    /// Language used
    pub language: String,
    /// Title of the challenge
    pub title: String,
}

/// Command to run
#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(command: &str, args: Vec<String>) -> Self {
        Self {
            command: command.to_string(),
            args,
        }
    }
}

/// Command to run
#[derive(Debug, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub content: Option<String>,
}

impl File {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: Some(content.to_string()),
        }
    }

    pub fn empty(name: &str) -> Self {
        Self {
            name: name.to_string(),
            content: None,
        }
    }
}

/// Programming template
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ProgrammingTemplate {
    /// Programming language
    pub language: String,

    /// Commands to run
    pub init_commands: Vec<Command>,

    /// Commands to run
    pub commands: Vec<Command>,

    /// Files to create
    pub files: Vec<File>,

    /// Folders to create
    pub folders: Vec<String>,
}

impl Display for DayChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Day: {}\nYear: {}\nLanguage: {}\nTitle: {}",
            self.day, self.year, self.language, self.title
        )
    }
}

impl DayChallenge {
    /// Create a new DayChallenge
    pub fn new(day: u8, year: u16, language: String, title: String) -> Self {
        Self {
            day,
            year,
            language: language.trim().to_lowercase(),
            title,
        }
    }
}
