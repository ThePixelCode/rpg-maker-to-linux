use std::{
    env::{args, current_dir},
    fs::File,
    io::Read,
    path::PathBuf,
    process::exit,
};

use errors::Errors;

use crate::config::Config;

mod config;
mod errors;

pub fn print_error_and_gracefully_exit(error: Errors) -> ! {
    println!("Error happened: {}", error);
    exit(1);
}

pub fn do_stuff() -> Result<(), Errors> {
    let mut working_directory = current_dir()?;
    let args = args();
    for arg in args {
        let path = PathBuf::from(arg);
        if path.exists() && path.is_dir() {
            working_directory = path.canonicalize()?;
        }
    }
    if !working_directory.join("nw.dll").exists() {
        return Err(Errors::UnknownFolder(
            working_directory.display().to_string(),
            "Maybe this is not a RPG Maker Game",
        ));
    }
    let config_file = working_directory.join("config.json");
    let mut config_file = File::open(&config_file)?;
    let mut json = Vec::new();
    config_file.read_to_end(&mut json)?;
    let config: Config = serde_json::from_slice(&json)?;
    // TODO: Actualy execute commands
    for command in config.pre_operation_commands {
        println!("{}", command);
    }
    for asociation in config.file_asociations {
        for destination in asociation.destination_files {
            println!(
                "ln {} {} {}",
                &asociation.allows_symlink, &asociation.origin_file, destination
            );
        }
    }
    for nwjs in config.checked_nwjs_versions {
        println!("curl {}", nwjs.nwjs_version);
        for command in nwjs.especific_nwjs_commands {
            println!("{}", command);
        }
    }
    for command in config.post_operation_commands {
        println!("{}", command);
    }
    Ok(())
}
