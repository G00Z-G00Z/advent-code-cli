use advent_code_cli::{interfaces::DayChallenge, yaml_parser};

const template_name: &str = "template.yml";
fn main() {
    let metadata = DayChallenge::new(1, 2020, "Rust".to_string(), "martin".to_string());

    let parsed_content =
        yaml_parser::parse_yml_file(template_name, &metadata).expect("Problem with parsing");

    println!("{}", parsed_content)
}
