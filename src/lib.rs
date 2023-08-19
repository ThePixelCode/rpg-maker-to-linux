use std::{fs, path, process, io, fmt::Display};

use errors::Errors;

mod config;
pub mod errors;
pub mod processor;

const NWJS_URL: &str = "https://dl.nwjs.io";
const NWJS_NORMAL_URL_FORMAT: &str = "{url}/{version}/nwjs-{version}-linux-x64.tar.gz";
const NWJS_SDK_URL_FORMAT: &str = "{url}/{version}/nwjs-sdk-{version}-linux-x64.tar.gz";

pub fn print_error_and_gracefully_exit(error: Errors) -> ! {
    println!("Error happened: {}", error);
    process::exit(1);
}

pub fn get_default_or_error<T: Default>(object_to_default: &str) -> Result<T, Errors> {
    if get_user_input(&format!("Warning: {} is undefined, do you wish to use it's default value", object_to_default))? {
        return Ok(T::default());
    }
    Err(Errors::UserCancelled)
}

pub fn get_user_input(prompt: &str) -> Result<bool, Errors> {
    println!("{} [Y/n]", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();

    if response == "y" || response.is_empty() {
        return Ok(true);
    } else if response == "n" {
        return Ok(false);
    }
    Err(Errors::Unknown)
}

pub fn get_user_input_with_choices<T: Display>(prompt: &str, choices: &Vec<(usize, T)>) -> Result<T, Errors> {
    if choices.is_empty() {
        return Err(Errors::Unknown);
    }

    println!("{}:", prompt);
    for (key, choice) in choices.iter() {
        println!("{}. {}", &key, &choice);
    }
    todo!()
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
