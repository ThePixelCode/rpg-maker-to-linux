use std::{fs, io, io::Read, os::unix, path, process};

use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use tar::Archive;

use crate::{
    config::Config, copy_files_recursively, errors::Errors, NWJS_NORMAL_URL_FORMAT, NWJS_URL, get_default_or_error,
};

pub fn check_directory_and_get_data(working_directory: path::PathBuf) -> Result<Data, Errors> {
    if !working_directory.join("nw.dll").exists() {
        return Err(Errors::UnknownFolder(working_directory.display().to_string(), "Maybe this is not an RPG Maker  Game"));
    }
    let config = match working_directory.join("config.json").exists(){
        true => {
            let config_file = working_directory.join("config.json");
            let mut file = fs::File::open(&config_file)?;
            let mut json = Vec::new();
            file.read_to_end(&mut json)?;
            serde_json::from_slice::<Config>(&json)?
        },
        false => {
            println!("No config found, Use default? [Y/n]");

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let response = input.trim().to_lowercase();

            if response != "y" {
                return Err(Errors::UserCancelled);
            }
            Config::default()
        }
    };
    Ok(Data::new(working_directory, config))
}

pub fn check_and_correct_data(data: &mut Data) -> Result<(), Errors> {
    if data.config.checked_nwjs_versions.is_empty() {
        data.config.checked_nwjs_versions.push(get_default_or_error("Checked NWJS Version")?);
    }
    for association in &data.config.file_asociations {
        if association.destination_files.is_empty() {
            return Err(Errors::MissingFileAssociations);
        }
    }
    Ok(())
}

pub fn process(data: Data) -> Result<(), Errors> {
    let mut data = data;
    data.execute_pre_op()?;
    data.execute_asociated()?;
    data.execute_nwjs()?;
    data.execute_post_op()?;
    Ok(())
}

pub struct Data {
    working_directory: path::PathBuf,
    config: Config,
}

impl Data {
    pub fn new(working_directory: path::PathBuf, config: Config) -> Self {
        Data {working_directory, config}
    }

    fn execute_pre_op(&self) -> Result<(), Errors> {
        for command in &self.config.pre_operation_commands {
            print!("Running: {}...", &command);
            let output = process::Command::new("sh")
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
                    unix::fs::symlink(
                        self.working_directory.join(&asociation.origin_file),
                        self.working_directory.join(destination),
                    )?;
                } else {
                    fs::hard_link(
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
            let output = process::Command::new("sh")
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

    fn download_file_to_dir(
        &self,
        version: &str,
        target_dir: &str,
        target_file: &str,
    ) -> Result<(), Errors> {
        let url = NWJS_NORMAL_URL_FORMAT
            .replace("{url}", NWJS_URL)
            .replace("{version}", version);

        fs::create_dir_all(&target_dir)?;
        let target_path = path::Path::new(target_dir).join(target_file);

        let mut response = Client::new().get(url).send()?;
        let mut file = fs::File::create(&target_path)?;

        io::copy(&mut response, &mut file)?;
        file.sync_all()?;

        Ok(())
    }

    fn extract_file_to_dir(&self, file: &str, target_dir: &str) -> Result<(), Errors> {
        let file_path = path::Path::new(file);
        let file = fs::File::open(file_path)?;
        let target_dir = path::Path::new(target_dir);

        fs::create_dir_all(&target_dir)?;

        let gz_decoder = GzDecoder::new(file);
        let mut tar_archive = Archive::new(gz_decoder);

        tar_archive.unpack(&target_dir)?;
        Ok(())
    }

    fn remove_dir_all(&self, target_dir: &str) -> Result<(), Errors> {
        let target_dir = path::Path::new(target_dir);

        fs::remove_dir_all(&target_dir)?;
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

            self.download_file_to_dir(&version, "/tmp/rpg2linux", "nwjs.tar.gz")?;

            println!("Download Completed");

            self.extract_file_to_dir("/tmp/rpg2linux/nwjs.tar.gz", "/tmp/rpg2linux/nwjs")?;

            println!("Extraction Completed");

            copy_files_recursively(
                path::Path::new(&format!(
                    "/tmp/rpg2linux/nwjs/nwjs-{version}-linux-x64",
                    version = &version
                )),
                &self.working_directory,
            )?;

            println!("Movement Completed");

            self.remove_dir_all("/tmp/rpg2linux")?;
        }
        Ok(())
    }
}
