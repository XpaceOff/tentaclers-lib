use std::fs;
use tracing::{error, instrument, trace, warn};

// Check for a list of folders that will be needed to run the program correctly
// If `file_creation` is true and the folder is not present, then it will create
// the folders, but it will error anyway.
#[instrument]
pub fn checker_core_folders(file_creation: bool) -> Result<(), &'static str> {
    let list_of_folders: Vec<&str> = vec!["./core", "./core/version", "./core/plugins"];

    let mut errors_out = false;

    for folder in list_of_folders.iter() {
        trace!("checking core folder {}", folder);

        match fs::metadata(folder) {
            // If the file exist but it is not a directory:
            Ok(metadata) => {
                if !metadata.is_dir() {
                    warn!("{} exist but it is not a directory.", folder);
                    errors_out = true;
                }
            }
            _ => {
                if file_creation {
                    match fs::create_dir(folder) {
                        Ok(_) => warn!("{} did not exist, but it just got created", folder),
                        Err(err_msg) => warn!(
                            " {} does not exist and can't be created. {}",
                            folder, err_msg
                        ),
                    }
                } else {
                    warn!("{} does not exist.", folder);
                };

                errors_out = true;
            }
        }
    }

    if errors_out {
        error!("There are core directories missing, the program won't run with out them.");
        return Err("There are core directories missing, the program won't run with out them.");
    }

    Ok(())
}
