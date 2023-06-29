use std::path::PathBuf;

/// Lists the folders in the directory
/// Only outputs the last of their name
pub fn list_folder_names(path: &PathBuf) {
    for entry in std::fs::read_dir(&path).unwrap() {
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
