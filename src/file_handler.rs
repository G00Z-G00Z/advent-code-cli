use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempdir::TempDir;

use crate::interfaces::ProgrammingTemplate;

#[derive(Debug)]
pub enum BuildError {
    FileError,
    TemplateError,
    DirectoryError,
}

const FIRST_YEAR: u32 = 2015;

pub struct AventStructure {
    base_directory: PathBuf,
}

impl AventStructure {
    pub fn new(base_directory: &str) -> AventStructure {
        AventStructure {
            base_directory: PathBuf::from(base_directory),
        }
    }

    pub fn initialize_year(&mut self, year: u32) -> Result<(), BuildError> {
        // Creates a folder structure fot the project
        // Creates a src folder
        // Creates folders made by years

        // Creates a src folder
        let src_path = self.base_directory.join("src");

        // Creates folders made by years
        let year_path = src_path.join(year.to_string());

        Ok(())
    }
}

pub fn build_template(
    template: &ProgrammingTemplate,
    base_directory: &str,
) -> Result<(), BuildError> {
    // Make the base directory a path instance
    let base_directory = PathBuf::from(base_directory);

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn get_tmp_dir() -> TempDir {
        TempDir::new("testing_cli").unwrap()
    }

    #[test]
    fn test_create_file() {
        // Create a temporary directory
        let temp_dir = get_tmp_dir();
        println!("The directory is located at {:?}", temp_dir.path());
        let base_dir = temp_dir.path();

        // Define the file path within the base directory
        let file_path: PathBuf = base_dir.join("my_file.txt");

        // Open the file and write some content
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, World!").unwrap();

        // Assert that the file exists
        assert!(file_path.exists());

        // Read and verify the file content
        let mut file_content = String::new();
        let mut file = File::open(&file_path).unwrap();
        file.read_to_string(&mut file_content).unwrap();
        assert_eq!(file_content, "Hello, World!");

        // Additional assertions or tests can be added here
    }
}
