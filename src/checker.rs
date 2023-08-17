use std::fs;
use tracing::instrument;

// Check for folder that will be needed to run the program correctly
// If `file_creation` is true, then it will create the folder, but it will error anyway.
//
#[instrument]
pub fn checker_core_folders(file_creation: bool) -> Result<(), String> {
    let list_of_folders: Vec<&str> = vec!["./version"];

    for folder in list_of_folders.iter() {
        match fs::metadata(folder) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err(format!(" {} is not a directory", folder));
                }
            }
            _ => {
                let r_error_msg = if file_creation {
                    match fs::create_dir(folder) {
                        Ok(_) => format!(" {} did not exist, but it was just created", folder),
                        Err(err_msg) => format!(
                            " {} does not exist and can't be created. {}",
                            folder, err_msg
                        ),
                    }
                } else {
                    format!(" {} does not exist.", folder)
                };

                return Err(r_error_msg);
            }
        }
    }

    Ok(())
}
