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

    let base_directory = cli
        .base_directory
        .unwrap_or_else(|| std::path::PathBuf::from("./"));

    println!("{:?}", base_directory);
}
