use clap::{Parser, Subcommand};
use std::{path::PathBuf, fmt::{Display, self, Formatter}};

pub enum CliError {
    BaseDirectoryError(String), 
    TemplateFileError(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CliError::BaseDirectoryError(e) => write!(f, "Base directory error: {}", e),
            CliError::TemplateFileError(e) => write!(f, "Template file error: {}", e),
        }
    }
}


#[derive(Parser)]
#[command(
    version = "0.1.0",
    author,
    about, 
    long_about = None
)]
pub struct Cli {
    /// The base directory to use. Will default to the current directory.
    #[arg(short, long)]
    pub base_directory: Option<PathBuf>,

    /// The template file with the languages
    #[arg(short, long)]
    pub template_file: Option<PathBuf>,

    /// Sub command to execute
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
    Add {
        /// The day to add
        #[arg(short, long)]
        day: u8,

        /// The year to add
        #[arg(short, long)]
        year: u16,

        /// The language to add
        #[arg(short, long)]
        language: String,

        /// Title of the challenge
        #[arg(short, long)]
        title: String,
    },
    Remove {

        /// The year to remove
        #[arg()]
        year: Option<u16>,

        /// The day to remove
        #[arg()]
        day: Option<u8>,

        /// The language to remove
        #[arg()]
        language: Option<String>,

    },
    /// Lists the challenges on a particular year
    List {
        /// The year to list. If none is provided, it will list all the years
        #[arg()]
        year: Option<u16>,
    },
}


impl Cli {


    pub fn init(&mut self) -> Result<(), CliError> {
        let base_directory = std::env::current_dir().unwrap();
        // Set the default base directory to the current directory
        if self.base_directory.is_none() {
            self.base_directory = Some(base_directory.clone());
        }

        // Check if the base directory exists
        if !self.base_directory.as_ref().unwrap().exists() {
            return Err(CliError::BaseDirectoryError(format!(
                "Base directory does not exist. Please select a new base directory: {:?}",
                self.base_directory
            )));
        }


        // Set the default template file to the current directory
        if self.template_file.is_none() {
            self.template_file = Some(base_directory.join("template.yml"));
        }

        // Check if the template file exists
        if !self.template_file.as_ref().unwrap().exists() {
            return Err(CliError::TemplateFileError(format!(
                "Template file does not exist. Create a new one to continue: {:?}",
                self.template_file
            )));
        }

        Ok(())
    }

}
