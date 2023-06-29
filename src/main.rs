use advent_code_cli::{
    cli::{Cli, Commands},
    utils::{list_folder_names, prompt_to_remove_directory},
};
use clap::Parser;
use std::{fs, path::PathBuf};

const TEMPLATE_NAME: &str = "template.yml";

fn main() {
    let mut cli = Cli::parse();
    match cli.init() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error! {}", e);
            return;
        }
    }

    let template_file = cli
        .template_file
        .unwrap_or_else(|| std::path::PathBuf::from(format!("./{}", TEMPLATE_NAME)));

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
                list_folder_names(&base_directory);
            }

            let year = year.unwrap();

            let year_path = base_directory.join(year.to_string());

            if !year_path.exists() {
                println!("You do not have entries for the year {}...", year);
                return;
            }

            // List all the days in the year
            println!("Listing all the days in the year: {}", year);
            list_folder_names(&year_path);
        }
        Commands::Remove {
            day,
            year,
            language,
            force,
        } => {
            let force = force.unwrap_or(false);

            let dir: PathBuf;
            match (year, day, language) {
                (None, None, None) => {
                    eprintln!("You need to specify: year, day, language");
                    return;
                }
                (Some(year), None, None) => {
                    dir = base_directory.join(year.to_string());
                }
                (Some(year), Some(day), None) => {
                    dir = base_directory.join(year.to_string()).join(day.to_string());
                }
                (Some(year), Some(day), Some(language)) => {
                    dir = base_directory
                        .join(year.to_string())
                        .join(day.to_string())
                        .join(language);
                }
                (Some(_), None, Some(_)) => unreachable!(),
                (None, _, _) => unreachable!(),
            };

            if !dir.exists() {
                eprintln!("The directory does not exist");
                return;
            }

            prompt_to_remove_directory(&dir, force);
        }
        Commands::Add {
            day,
            year,
            language,
            title,
        } => {
            todo!()
        }
    }
}
