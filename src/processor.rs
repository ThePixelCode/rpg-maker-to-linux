use std::{
    env::{args, current_dir},
    fs::{hard_link, File},
    io::Read,
    os::unix::fs,
    path::PathBuf,
    process::Command,
};

use crate::{config::Config, errors::Errors};

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

    fn execute_nwjs(&self) -> Result<(), Errors> {
        for nwjs in &self.config.checked_nwjs_versions {
            println!("curl {}", nwjs.nwjs_version);
            for command in &nwjs.especific_nwjs_commands {
                println!("{}", command);
            }
        }
        Ok(())
    }

    pub fn execute(&self) -> Result<(), Errors> {
        self.check_conditions()?;
        self.execute_pre_op()?;
        self.execute_asociated()?;
        self.execute_nwjs()?;
        self.execute_post_op()?;
        Ok(())
    }
}
