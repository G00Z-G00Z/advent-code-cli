use std::{fs, path::PathBuf};

use crate::interfaces::{DayChallenge, ProgrammingTemplate};

#[derive(Debug)]
pub enum BuildError {
    FileError,
    TemplateError(String),
    DirectoryError,
}

/// Hanldes the creation of the directory structure for the advent of code
pub struct AventStructure {
    pub base_directory: PathBuf,
}

impl AventStructure {
    pub fn new(base_directory: PathBuf) -> AventStructure {
        AventStructure {
            base_directory: base_directory.join("src"),
        }
    }

    /// Creates the directory of a day challenge structure for the advent of code
    pub fn add_day(
        &mut self,
        challenge: &DayChallenge,
        template: &ProgrammingTemplate,
    ) -> Result<PathBuf, BuildError> {
        let year_path = self
            .base_directory
            .join(challenge.year.to_string())
            .join(challenge.language.to_string())
            .join(format!("Day-{:02}-{}", challenge.day, challenge.title));
        fs::create_dir_all(&year_path).map_err(|_| BuildError::DirectoryError)?;

        // Run the init commands
        for c in &template.init_commands {
            let ouput = std::process::Command::new("sh")
                .arg("-c")
                .arg(&c.command)
                .current_dir(&year_path)
                .output()
                .map_err(|_| {
                    BuildError::TemplateError(format!("{} failed to execute", c.command))
                })?;

            if !ouput.status.success() {
                return Err(BuildError::TemplateError(format!(
                    "{} failed to execute",
                    c.command
                )));
            }
        }

        // Create the folders
        for folder in &template.folders {
            let folder_path = year_path.join(folder);
            fs::create_dir_all(&folder_path).map_err(|_| BuildError::DirectoryError)?;
        }

        // Create the files
        for f in &template.files {
            let file_path = year_path.join(&f.name);
            fs::write(&file_path, &f.content.clone().unwrap_or("".to_string()))
                .map_err(|_| BuildError::FileError)?;
        }

        // Run the commands
        for c in &template.commands {
            let ouput = std::process::Command::new("sh")
                .arg("-c")
                .arg(&c.command)
                .current_dir(&year_path)
                .output()
                .map_err(|e| {
                    BuildError::TemplateError(format!("{} failed to execute\n\n{:?}", c.command, e))
                })?;

            if !ouput.status.success() {
                return Err(BuildError::TemplateError(format!(
                    "{} failed to execute\n\n{}",
                    c.command,
                    String::from_utf8_lossy(&ouput.stderr)
                )));
            }
        }

        Ok(year_path)
    }
}

#[cfg(test)]
mod tests {
    use crate::interfaces::{Command, File};

    use super::*;

    use tempdir::TempDir;

    fn get_tmp_dir() -> TempDir {
        TempDir::new("testing_cli").unwrap()
    }

    fn get_challenge() -> DayChallenge {
        DayChallenge {
            day: 1,
            year: 2015,
            language: "rust".to_string(),
            title: "test-my-challenge".to_string(),
        }
    }

    fn get_template() -> ProgrammingTemplate {
        ProgrammingTemplate {
            language: "rust".to_string(),
            init_commands: vec![Command::new("cargo init . --vcs none", vec![])],
            commands: vec![Command::new("cargo build", vec![])],
            files: vec![
                File::empty("input.txt"),
                File::new("demo-input.txt", "demo"),
            ],
            folders: vec!["docs/".to_string()],
        }
    }

    #[test]
    fn test_initialize_year_folder_correctly() {
        let base_dir = get_tmp_dir().as_ref().join("src");
        let challenge = get_challenge();
        let template = get_template();
        const YEAR: u32 = 2015;
        let base_dir_c = base_dir.clone();
        let mut structure = AventStructure::new(base_dir);
        let base_dir = base_dir_c;

        let new_dir = structure.add_day(&challenge, &template).unwrap();
        println!("{:?}", new_dir);

        let challenge_path = base_dir
            .join("src")
            .join(YEAR.to_string())
            .join(challenge.language.to_string())
            .join(format!("Day-{:02}-{}", challenge.day, challenge.title));

        println!("{:?}", challenge_path);

        assert!(
            challenge_path.exists(),
            "Year {:?} directory was not created",
            challenge_path
        );
        assert_eq!(challenge_path, new_dir);

        // make file assertions
        // Check for cargo init

        let cargo_toml = challenge_path.join("Cargo.toml");
        assert!(cargo_toml.exists(), "Cargo.toml was not created");
        let cargo_lock = challenge_path.join("Cargo.lock");
        assert!(cargo_lock.exists(), "Cargo.lock was not created");

        // Check for input.txt
        let input_txt = challenge_path.join("input.txt");
        assert!(input_txt.exists(), "input.txt was not created");
        let input_txt_content = fs::read_to_string(&input_txt).unwrap();
        assert_eq!(input_txt_content, "", "Input content is wrong");

        // Check for demo-input.txt
        let demo_input_txt = challenge_path.join("demo-input.txt");
        assert!(demo_input_txt.exists(), "demo-input.txt was not created");
        let demo_input_txt_content = fs::read_to_string(&demo_input_txt).unwrap();
        assert_eq!(
            demo_input_txt_content, "demo",
            "demo-input.txt content is wrong"
        );

        // check that source exists
        let src_path = challenge_path.join("src");
        assert!(src_path.exists(), "src was not created");
        let main_path = src_path.join("main.rs");
        assert!(main_path.exists(), "main.rs was not created");

        // Check that docs exists
        let docs_path = challenge_path.join("docs");
        assert!(docs_path.exists(), "docs was not created");

        // Check that target folder exists after cargo build
        let target_path = challenge_path.join("target");
        assert!(target_path.exists(), "Cargo build was not ran");

        // Check that no git folder was created
        let git_path = challenge_path.join(".git");
        assert!(!git_path.exists(), ".git folder was created");
    }
}
