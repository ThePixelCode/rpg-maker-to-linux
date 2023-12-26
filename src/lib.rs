pub mod args;
pub mod logger;
pub mod port;
pub mod run;

pub const NWJS_VERSION: &str = "v0.83.0";

pub const DEBUG_ERROR: u8 = 0;
pub const DEBUG_WARN: u8 = 1;
pub const DEBUG_INFO: u8 = 2;

pub struct GameData<P: AsRef<std::path::Path>> {
    path: P,
    file: std::fs::File,
    sdk: bool,
}

impl<P: AsRef<std::path::Path>> GameData<P> {
    pub fn new(path: P, sdk: bool) -> Result<Self, std::io::Error> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(path.as_ref().join("package.json"))?;
        Ok(GameData { path, file, sdk })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GeneralErrors {
    #[error("Error getting cache folder")]
    GettingCacheFolderError,
    #[error("Error found while setting cache folders error was {0}")]
    SettingCacheFolderError(#[from] std::io::Error),
}

pub fn get_cache_folder() -> Result<std::path::PathBuf, GeneralErrors> {
    let user = users::get_current_username()
        .ok_or(GeneralErrors::GettingCacheFolderError)?
        .into_string()
        .map_err(|_| GeneralErrors::GettingCacheFolderError)?;
    let binding = format!("/home/{}/.cache/rpg2linux/", user);
    let path = std::path::Path::new(&binding);
    if !path.exists() {
        std::fs::create_dir_all(path)?
    }
    Ok(path.to_path_buf())
}
