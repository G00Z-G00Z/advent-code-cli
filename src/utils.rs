use dialoguer::Confirm;
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

/// Prompts user to remove a directory
/// If force is true, it will remove it without prompting
pub fn prompt_to_remove_directory(path: &PathBuf, force: bool) {
    if force {
        std::fs::remove_dir_all(path).unwrap();
        println!("Removed {}", path.display());
        return;
    }

    let confirmed = Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to delete {}?",
            path.display()
        ))
        .interact()
        .expect("Canceling operation");

    if confirmed {
        // Proceed with the desired action
        std::fs::remove_dir_all(path).unwrap();
        println!("Removed {}", path.display());
    } else {
        println!("Canceling operation");
    }
}
