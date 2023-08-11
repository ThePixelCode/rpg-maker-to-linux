use std::{fs, path, process};

use errors::Errors;

mod config;
mod errors;
pub mod processor;

const NWJS_URL: &str = "https://dl.nwjs.io";
const NWJS_NORMAL_URL_FORMAT: &str = "{url}/{version}/nwjs-{version}-linux-x64.tar.gz";
const NWJS_SDK_URL_FORMAT: &str = "{url}/{version}/nwjs-sdk-{version}-linux-x64.tar.gz";

pub fn print_error_and_gracefully_exit(error: Errors) -> ! {
    println!("Error happened: {}", error);
    process::exit(1);
}

pub fn copy_files_recursively(
    source_path: &path::Path,
    target_path: &path::Path,
) -> Result<(), Errors> {
    if source_path.is_file() {
        fs::copy(&source_path, &target_path)?;
    } else if source_path.is_dir() {
        fs::create_dir_all(&target_path)?;

        for entry in fs::read_dir(&source_path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let target_path = target_path.join(entry_path.file_name().ok_or(Errors::Unknown)?);

            copy_files_recursively(&entry_path, &target_path)?;
        }
    }
    Ok(())
}
