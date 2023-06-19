use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::interfaces::DayChallenge;

#[derive(Debug)]
pub enum BuildError {
    FileError,
    TemplateError,
    DirectoryError,
}

/// Hanldes the creation of the directory structure for the advent of code
pub struct AventStructure {
    base_directory: PathBuf,
}

impl AventStructure {
    pub fn new(base_directory: &Path) -> AventStructure {
        AventStructure {
            base_directory: PathBuf::from(base_directory),
        }
    }

    /// Creates the directory of a day challenge structure for the advent of code
    pub fn add_day(&mut self, challenge: &DayChallenge) -> Result<PathBuf, BuildError> {
        let year_path = self
            .base_directory
            .join("src")
            .join(challenge.year.to_string())
            .join(format!("Day-{:02}-{}", challenge.day, challenge.title))
            .join(challenge.language.to_string());
        fs::create_dir_all(&year_path).map_err(|_| BuildError::DirectoryError)?;
        Ok(year_path)
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_initialize_year_folder_correctly() {
        let base_dir = get_tmp_dir();
        let challenge = get_challenge();
        const YEAR: u32 = 2015;
        let mut structure = AventStructure::new(base_dir.as_ref());

        let new_dir = structure.add_day(&challenge).unwrap();

        let challenge_path = base_dir
            .path()
            .join("src")
            .join(YEAR.to_string())
            .join(format!("Day-{:02}-{}", challenge.day, challenge.title))
            .join(challenge.language.to_string());

        assert!(
            challenge_path.exists(),
            "Year {:?} directory was not created",
            challenge_path
        );
        assert_eq!(challenge_path, new_dir);
    }
}
