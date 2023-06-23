use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

impl Cli {
    pub fn init(&mut self) {
        let base_directory = std::env::current_dir().unwrap();
        // Set the default base directory to the current directory
        if self.base_directory.is_none() {
            self.base_directory = Some(base_directory.clone());
        }

        // Set the default template file to the current directory
        if self.template_file.is_none() {
            self.template_file = Some(base_directory.join("template.yml"));
        }
    }
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
        /// The day to remove
        #[arg(short, long)]
        day: Option<u8>,

        /// The year to remove
        #[arg(short, long)]
        year: Option<u16>,

        /// The language to remove
        #[arg(short, long)]
        language: Option<String>,
    },
    List {
        /// The year to list
        #[arg(short, long)]
        year: Option<u16>,
    },
}
