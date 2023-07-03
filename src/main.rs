use advent_code_cli::{
    cli::{Cli, Commands},
    file_handler::{AventStructure, BuildError},
    interfaces::DayChallenge,
    utils::{list_folder_names, prompt_to_remove_directory},
    yaml_parser::{parse_values_yml, populate_yml, YamlParserError},
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

    let mut structure = AventStructure::new(base_directory);

    match cli.command {
        Commands::List { year, lang } => {
            if year.is_none() {
                // List all the years in the base directory
                println!("Listing all the years in the base directory: ");
                list_folder_names(&structure.base_directory);
                return ();
            }

            let year = year.unwrap();

            let year_path = structure.base_directory.join(year.to_string());

            if !year_path.exists() {
                println!("You do not have entries for the year {}...", year);
                return;
            }

            // Check if only one language

            let langs = std::fs::read_dir(&year_path)
                .expect("Expected to be able to read year")
                .filter_map(|e| {
                    let path = e.unwrap().path();
                    if path.is_dir() {
                        Some(path.file_name().unwrap().to_str().unwrap().to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();

            if langs.is_empty() {
                println!("You do not have entries for the year {}...", year);
                return;
            }

            if langs.iter().count() == 1 {
                println!(
                    "Listing all the days from the year {} in the language {}",
                    year, langs[0]
                );
                let path = year_path.join(&langs[0]);
                list_folder_names(&path);
            }

            if lang.is_none() {
                // List all the languages in the year
                println!("Listing all the languages in the year {}", year);
                list_folder_names(&year_path);
                return;
            }

            let lang = lang.unwrap();

            let lang_path = year_path.join(&lang);

            if !lang_path.exists() {
                println!(
                    "You do not have entries for the year {} in the language {}...",
                    year, lang
                );
                return;
            }

            // List all the days in the language

            println!(
                "Listing all the days from the year {} in the language {}",
                year, lang
            );

            list_folder_names(&lang_path);
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
                    dir = structure.base_directory.join(year.to_string());
                }
                (Some(year), Some(day), None) => {
                    dir = structure
                        .base_directory
                        .join(year.to_string())
                        .join(day.to_string());
                }
                (Some(year), Some(day), Some(language)) => {
                    dir = structure
                        .base_directory
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
            let day_challenge = DayChallenge {
                day,
                year,
                language,
                title,
            };

            let template_content =
                fs::read_to_string(&template_file).expect("Unable to read template file");

            let populated_template = match populate_yml(&template_content, &day_challenge) {
                Ok(template) => template,
                Err(e) => match e {
                    YamlParserError::TeraError(e) => {
                        panic!("Error! {}", e);
                    }
                    _ => unreachable!(),
                },
            };

            let programming_template =
                parse_values_yml(&populated_template, &day_challenge.language);

            if let Err(e) = programming_template {
                match e {
                    YamlParserError::NoLanguagesProvided => {
                        panic!("No languages provided in the template file");
                    }
                    YamlParserError::NoLanguageFound(l) => {
                        panic!("No language found for {} in template file", l);
                    }
                    YamlParserError::YamlError(e) => {
                        panic!("YamlError! {}", e);
                    }
                    YamlParserError::BadFormat(m) => {
                        panic!("Bad format! {}", m);
                    }
                    YamlParserError::TeraError(_) => unreachable!(),
                }
            }

            let programming_template = programming_template.unwrap();

            match structure.add_day(&day_challenge, &programming_template) {
                Ok(p) => {
                    println!("Successfully created the directory structure");
                    println!("Path: {}", p.as_os_str().to_str().unwrap());
                }
                Err(e) => match e {
                    BuildError::FileError => {
                        panic!("FileError: Problems building the files ");
                    }
                    BuildError::TemplateError(m) => {
                        panic!("TemplateError: {}", m);
                    }
                    BuildError::DirectoryError => {
                        panic!("DirectoryError: Problems building the directories");
                    }
                },
            }
        }
    }
}
