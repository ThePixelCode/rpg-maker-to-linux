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
    let folder = path.to_path_buf();
    let logs = folder.join("logs");
    if !logs.exists() {
        std::fs::create_dir_all(logs)?;
    }
    let download = folder.join("download");
    if !download.exists() {
        std::fs::create_dir_all(download)?;
    }
    Ok(folder)
}

pub fn parse_steam_args(args: Vec<String>) -> Vec<String> {
    let mut new_args = Vec::new();
    let mut program = false;
    let mut wanted = true;
    let mut arg_index = 3_i8;

    for arg in args {
        // FIXME: Make a more general solution
        if arg_index == 0 {
            new_args.push(String::from("--"));
        }
        if arg_index >= 0 {
            arg_index -= 1;
        }
        if arg != "--" && !program {
            if wanted {
                new_args.push(arg);
            }
            continue;
        }
        if arg == "--" {
            program = true;
            if wanted {
                new_args.push(arg);
            }
            wanted = true;
            continue;
        }
        if arg.contains("Steam/ubuntu12_32") {
            new_args.push(arg);
            program = false;
            continue;
        }
        if arg.contains("common/SteamLinuxRuntime") {
            program = false;
            wanted = false;
            continue;
        }
        if arg.contains("common/Proton") {
            wanted = false;
            continue;
        }
        if arg == "waitforexitandrun" {
            program = false;
            wanted = true;
            continue;
        }
        new_args.push(arg);
    }
    let last = new_args.pop().unwrap();
    let mut last = last.split("/").collect::<Vec<&str>>();
    let _ = last.pop();
    new_args.push(last.join("/"));
    new_args
}
