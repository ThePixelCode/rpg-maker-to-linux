use std::{
    env::{args, current_dir},
    fs::{create_dir_all, hard_link, read_dir, remove_dir_all, rename, File},
    io::{copy, Read, Write},
    os::unix::fs,
    path::{Path, PathBuf},
    process::Command,
};

use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use tar::Archive;

use crate::{config::Config, errors::Errors, NWJS_NORMAL_URL_FORMAT, NWJS_URL};

pub struct Process {
    working_directory: PathBuf,
    config: Config,
}

impl Process {
    pub fn new() -> Result<Self, Errors> {
        let mut working_directory = current_dir()?;
        let args = args();
        for arg in args {
            let path = PathBuf::from(arg);
            if path.exists() && path.is_dir() {
                working_directory = path.canonicalize()?;
            }
        }
        let config_file = working_directory.join("config.json");
        let mut config_file = File::open(&config_file)?;
        let mut json = Vec::new();
        config_file.read_to_end(&mut json)?;
        let config: Config = serde_json::from_slice(&json)?;
        Ok(Process {
            working_directory,
            config,
        })
    }

    fn check_conditions(&self) -> Result<(), Errors> {
        if !self.working_directory.join("nw.dll").exists() {
            return Err(Errors::UnknownFolder(
                self.working_directory.display().to_string(),
                "Maybe this is not a RPG Maker Game",
            ));
        }
        if self.config.checked_nwjs_versions.is_empty() {
            return Err(Errors::MissingNWJSVersions);
        }
        for association in &self.config.file_asociations {
            if association.destination_files.is_empty() {
                return Err(Errors::MissingFileAssociations);
            }
        }
        Ok(())
    }

    fn execute_pre_op(&self) -> Result<(), Errors> {
        for command in &self.config.pre_operation_commands {
            print!("Running: {}...", &command);
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(&self.working_directory)
                .output()?;
            if output.status.success() {
                println!("Ok");
            } else {
                println!("Error");
                return Err(Errors::ProcessError(
                    "pre_op",
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ));
            }
        }
        Ok(())
    }

    fn execute_asociated(&self) -> Result<(), Errors> {
        for asociation in &self.config.file_asociations {
            for destination in &asociation.destination_files {
                if asociation.allows_symlink {
                    fs::symlink(
                        self.working_directory.join(&asociation.origin_file),
                        self.working_directory.join(destination),
                    )?;
                } else {
                    hard_link(
                        self.working_directory.join(&asociation.origin_file),
                        self.working_directory.join(destination),
                    )?;
                }
            }
        }
        Ok(())
    }

    fn execute_post_op(&self) -> Result<(), Errors> {
        for command in &self.config.post_operation_commands {
            print!("Running: {}...", &command);
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(&self.working_directory)
                .output()?;
            if output.status.success() {
                println!("Ok");
            } else {
                println!("Error");
                return Err(Errors::ProcessError(
                    "pre_op",
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ));
            }
        }
        Ok(())
    }

    fn execute_nwjs(&mut self) -> Result<(), Errors> {
        self.config.checked_nwjs_versions.sort();
        if let Some(last) = self.config.checked_nwjs_versions.pop() {
            let version = last.get_version()?;
            println!("Last version checked is {}", &version);
            let versions: Vec<String> = self
                .config
                .checked_nwjs_versions
                .iter()
                .flat_map(|nwjs| nwjs.get_version())
                .collect();
            println!("Other checked versions are: {:#?}", &versions);
            let url = NWJS_NORMAL_URL_FORMAT
                .replace("{url}", NWJS_URL)
                .replace("{version}", &version);
            let target_dir = "/tmp/rpg2linux";
            let target_file = "nwjs.tar.gz";

            create_dir_all(&target_dir)?;
            let target_path = Path::new(target_dir).join(target_file);

            let mut response = Client::new().get(url).send()?;
            let mut file = File::create(&target_path)?;

            copy(&mut response, &mut file)?;

            file.sync_all()?;

            let target_folder = Path::new(target_dir).join("nwjs");

            create_dir_all(&target_folder)?;

            let gz_decoder = GzDecoder::new(file);
            let mut tar_archive = Archive::new(gz_decoder);

            tar_archive.unpack(&target_folder)?;

            for entry in read_dir(&target_folder)? {
                let entry = entry?;
                let source = entry.path();
                let target = self
                    .working_directory
                    .join(source.file_name().ok_or(Errors::Unknown)?);

                rename(source, target)?;
            }

            //remove_dir_all(&target_dir)?;
        }
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), Errors> {
        //self.check_conditions()?;
        self.execute_pre_op()?;
        //self.execute_asociated()?;
        self.execute_nwjs()?;
        self.execute_post_op()?;
        Ok(())
    }
}
