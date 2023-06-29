use std::fs;

use advent_code_cli::cli::{Cli, Commands};
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

    let base_directory = base_directory.join("src");
    fs::create_dir_all(&base_directory).unwrap();

    match cli.command {
        Commands::List { year } => {
            if year.is_none() {
                // List all the years in the base directory
                println!("Listing all the years in the base directory: ");
                for entry in std::fs::read_dir(&base_directory).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_dir() {
                        println!(
                            "- {}",
                            path.components()
                                .last()
                                .unwrap()
                                .as_os_str()
                                .to_str()
                                .unwrap()
                        );
                    }
                }
                return;
            }

            let year = year.unwrap();

            let year_path = base_directory.join(year.to_string());

            if !year_path.exists() {
                println!("You do not have entries for the year {}...", year);
                return;
            }

            // List all the days in the year
            println!("Listing all the days in the year: {}", year);

            for entry in std::fs::read_dir(&year_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    println!(
                        "- {}",
                        path.components()
                            .last()
                            .unwrap()
                            .as_os_str()
                            .to_str()
                            .unwrap()
                    );
                }
            }
        }
        Commands::Add {
            day,
            year,
            language,
            title,
        } => todo!(),
        Commands::Remove {
            day,
            year,
            language,
        } => todo!(),
    }
}
