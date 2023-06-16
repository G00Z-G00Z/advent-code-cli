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
#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

/// Programming template
#[derive(Debug)]
pub struct ProgrammingTemplate {
    /// Programming language
    pub language: String,

    /// Commands to run
    pub init_commands: Vec<Command>,

    /// Commands to run
    pub commands: Vec<Command>,

    /// Files to create
    pub files: Vec<String>,

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
            language,
            title,
        }
    }
}
