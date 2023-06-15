use advent_code_cli::{interfaces::DayChallenge, yaml_parser};

const TEMPLATE_NAME: &str = "template.yml";

fn main() {
    let content =
        std::fs::read_to_string(TEMPLATE_NAME).expect("Something went wrong reading the file");
    let metadata = DayChallenge::new(1, 2020, "Rust".to_string(), "martin".to_string());

    let parsed_content =
        yaml_parser::parse_yml_file(&content, &metadata).expect("Problem with parsing");

    println!("{}", parsed_content)
}
