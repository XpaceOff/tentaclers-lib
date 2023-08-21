use regex::Regex;
use std::{ffi::OsString, fs, path::PathBuf};
use tracing::{debug, error, instrument, trace, warn};

// Check for a list of folders that will be needed to run the program correctly
// If `file_creation` is true and the folder is not present, then it will create
// the folders, but it will error anyway.
#[instrument]
pub fn check_core_dirs(file_creation: bool) -> Result<(), &'static str> {
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
        let err_msg = "There are core directories missing, the program won't run with out them.";
        error!(err_msg);

        return Err(err_msg);
    }

    Ok(())
}

#[instrument]
pub fn check_api_ver_dir() -> Result<Vec<File>, String> {
    let api_ver_path = "./core/version/";
    let mut r_list: Vec<File> = vec![];

    // Read the file directory
    let file_list = match fs::read_dir(api_ver_path) {
        Ok(f_list) => f_list,
        Err(err_msg) => {
            let err_msg = format!(
                " The directory {} is not accessible. {}",
                api_ver_path, err_msg
            );

            error!("{}", &err_msg);
            return Err(err_msg);
        }
    };

    // Go through each file
    for n_file in file_list {
        trace!("{:?}", n_file);
        let current_file = match n_file {
            Ok(dir_entry) => {
                if let Ok(metadata) = dir_entry.metadata() {
                    File::new(dir_entry.file_name(), dir_entry.path(), metadata)
                } else {
                    warn!(
                        " Metadata of file {} can't be read it",
                        dir_entry.file_name().to_string_lossy()
                    );
                    continue;
                }
            }
            Err(err_msg) => {
                warn!(" {} ", err_msg);
                continue;
            }
        };

        // Pass if it is not a directory
        if !current_file.metadata.is_dir() {
            warn!(
                " File '{}' was found in '{}' This directory should only include folders",
                current_file.name, api_ver_path
            );
            continue;
        }

        // If it is a folder then lets check if it has the format vXX
        let ver_pattern =
            Regex::new(r"^v[1-9]\d*$").expect("Something unexpected happened to the regex pattern");

        if ver_pattern.is_match(&current_file.name) {
            r_list.push(current_file);
        } else {
            warn!(
                "'{}' is not a valid api entry. It will be ignored",
                current_file.name
            );
        }
    }

    if r_list.len() > 0 {
        Ok(r_list)
    } else {
        error!(
            " The directory '{}' is empty or doesn't have any valid path.",
            api_ver_path
        );

        Err(format!(
            " The directory '{}' is empty or doesn't have any valid path.",
            api_ver_path
        ))
    }
}

pub struct File {
    name: String, //OsString
    path: PathBuf,
    metadata: fs::Metadata,
}

impl File {
    pub fn new(name: OsString, path: PathBuf, metadata: fs::Metadata) -> Self {
        File {
            name: name.to_string_lossy().into_owned(),
            path,
            metadata,
        }
    }
}
