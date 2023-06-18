use advent_code_cli::{
    interfaces::DayChallenge,
    yaml_parser::{self, parse_values_yml},
};

const TEMPLATE_NAME: &str = "template.yml";

fn main() {
    let content =
        std::fs::read_to_string(TEMPLATE_NAME).expect("Something went wrong reading the file");
    let metadata = DayChallenge::new(1, 2020, "Rust".to_string(), "martin".to_string());

    let parsed_content =
        yaml_parser::populate_yml(&content, &metadata).expect("Problem with parsing");

    match parse_values_yml(&parsed_content, &metadata.language) {
        Ok(value) => println!("Parsed value: {:?}", value),
        Err(e) => println!("Error: {:?}", e),
    };
}
