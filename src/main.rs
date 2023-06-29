use advent_code_cli::{
    cli::Cli,
    interfaces::DayChallenge,
    yaml_parser::{self, parse_values_yml},
};
use clap::Parser;

const TEMPLATE_NAME: &str = "template.yml";

fn main() {
    let mut cli = Cli::parse();
    cli.init();

    let template_file = cli
        .template_file
        .unwrap_or_else(|| std::path::PathBuf::from("./template.yml"));

    // Check if the template file exists
    if !template_file.exists() {
        eprintln!("Template file does not exist. Create a new one to continue");
        return;
    }

    let base_directory = cli
        .base_directory
        .unwrap_or_else(|| std::path::PathBuf::from("./"));

    // Check if the base directory exists
    if !base_directory.exists() {
        eprintln!("Base directory does not exist. Please select a new base directory");
        return;
    }
}
