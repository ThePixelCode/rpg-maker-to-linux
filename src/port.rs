#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct PackageWindow {
    title: String,
    toolbar: bool,
    width: u32,
    height: u32,
    icon: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Package {
    name: String,
    main: String,
    #[serde(rename = "js-flags")]
    js_flags: String,
    window: PackageWindow,
}

enum CheckResult {
    Ok,
    ErrorsFound(Package),
}

#[derive(thiserror::Error, Debug)]
pub enum PortingErrors {
    #[error("IO error found! The error was {0}")]
    IOError(#[from] std::io::Error),
    #[error("JSON error found! The error was {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Internet error found! The error was {0}")]
    InternetError(#[from] reqwest::Error),
    #[error("General error found! The error was {0}")]
    GeneralError(#[from] crate::GeneralErrors),
    #[error("Unknown error")]
    UnknownError,
}

type Result<T> = std::result::Result<T, PortingErrors>;

fn check_package_json<P>(
    game_data: &mut crate::GameData<P>,
    logger: &mut crate::logger::Logger,
) -> Result<CheckResult>
where
    P: AsRef<std::path::Path>,
{
    let mut buffer = String::new();
    std::io::Read::read_to_string(&mut game_data.file, &mut buffer)?;
    let package = serde_json::from_str::<Package>(&buffer)?;
    logger.log("package.json succesfully read");
    if package.name.trim().is_empty() {
        logger.warn("name is not defined");
        return Ok(CheckResult::ErrorsFound(package));
    }
    Ok(CheckResult::Ok)
}

fn do_package_json_corrections<P>(
    game_data: &mut crate::GameData<P>,
    mut package: Package,
    logger: &mut crate::logger::Logger,
) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    package.name = String::from("asd");
    logger.log(&format!("setting name to {}", &package.name));
    let package_string = serde_json::to_string_pretty(&package)?;
    std::io::Seek::rewind(&mut game_data.file)?;
    std::io::Write::write_all(&mut game_data.file, package_string.as_bytes())?;
    game_data.file.sync_all()?;
    logger.log("succesfully corrected package.json");
    Ok(())
}

fn copy_files_recursively(
    source_path: &std::path::Path,
    target_path: &std::path::Path,
    logger: &mut crate::logger::Logger,
) -> Result<()> {
    // logger.log(&format!(
    //     "copied {} to {}",
    //     &source_path.display(),
    //     &target_path.display()
    // ));
    if source_path.is_file() {
        std::fs::copy(&source_path, &target_path)?;
    } else if source_path.is_dir() {
        std::fs::create_dir_all(&target_path)?;

        for entry in std::fs::read_dir(&source_path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let target_path =
                target_path.join(entry_path.file_name().ok_or(PortingErrors::UnknownError)?);

            copy_files_recursively(&entry_path, &target_path, logger)?;
        }
    }
    Ok(())
}

fn patch_game<P>(
    game_data: &mut crate::GameData<P>,
    logger: &mut crate::logger::Logger,
) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    let folder = crate::get_cache_folder()?.join("download");
    std::fs::create_dir_all(folder.clone())?;
    let size = match std::fs::metadata(folder.join(match game_data.sdk {
        true => format!("nwjs-sdk-{}-linux-x64.tar.gz", crate::NWJS_VERSION),
        false => format!("nwjs-{}-linux-x64.tar.gz", crate::NWJS_VERSION),
    })) {
        Ok(metadata) => metadata.len(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => 0,
        Err(e) => Err(e)?,
    };

    let mut local_file = match size {
        0 => {
            let url = match game_data.sdk {
                true => {
                    format!(
                        "https://dl.nwjs.io/{0}/nwjs-sdk-{0}-linux-x64.tar.gz",
                        crate::NWJS_VERSION
                    )
                }
                false => {
                    format!(
                        "https://dl.nwjs.io/{0}/nwjs-{0}-linux-x64.tar.gz",
                        crate::NWJS_VERSION
                    )
                }
            };
            logger.log(&format!("downloading {}", &url));
            let mut file = reqwest::blocking::get(url)?;
            if !file.status().is_success() {
                return Err(PortingErrors::UnknownError);
            }
            let mut local_file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(folder.join(match game_data.sdk {
                    true => format!("nwjs-sdk-{}-linux-x64.tar.gz", crate::NWJS_VERSION),
                    false => format!("nwjs-{}-linux-x64.tar.gz", crate::NWJS_VERSION),
                }))?;

            let size = file.copy_to(&mut local_file)?;
            local_file.sync_all()?;
            logger.log(&format!("downloaded {} bytes", size));
            local_file
        }
        _ => {
            logger.warn("cache found, using it, if fails please delete .cache/rpg2linux/downloads");
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(folder.join(match game_data.sdk {
                    true => format!("nwjs-sdk-{}-linux-x64.tar.gz", crate::NWJS_VERSION),
                    false => format!("nwjs-{}-linux-x64.tar.gz", crate::NWJS_VERSION),
                }))?
        }
    };

    let folder = crate::get_cache_folder()?.join(match game_data.sdk {
        true => format!("nwjs-sdk-{}-linux-x64", crate::NWJS_VERSION),
        false => format!("nwjs-{}-linux-x64", crate::NWJS_VERSION),
    });
    std::fs::create_dir_all(folder.clone())?;

    if folder.read_dir()?.count() < 1 {
        std::io::Seek::rewind(&mut local_file)?;
        let gz_decoder = flate2::read::GzDecoder::new(local_file);
        let mut archive = tar::Archive::new(gz_decoder);
        archive.unpack(folder.clone())?;
        logger.log("unpack completed");
    } else {
        logger.warn(&format!(
            "cache found, using it, if fails please delete .cache/rpg2linux/{}",
            match game_data.sdk {
                true => format!("nwjs-sdk-{}-linux-x64", crate::NWJS_VERSION),
                false => format!("nwjs-{}-linux-x64", crate::NWJS_VERSION),
            }
        ));
    }

    copy_files_recursively(
        &folder.join(match game_data.sdk {
            true => format!("nwjs-sdk-{}-linux-x64", crate::NWJS_VERSION),
            false => format!("nwjs-{}-linux-x64", crate::NWJS_VERSION),
        }),
        game_data.path.as_ref(),
        logger,
    )?;
    logger.log(&format!(
        "copied extracted files to {}",
        game_data.path.as_ref().display()
    ));

    Ok(())
}

pub fn port<P>(game_data: &mut crate::GameData<P>, logger: &mut crate::logger::Logger) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    match check_package_json(game_data, logger)? {
        CheckResult::Ok => logger.log("package.json is correct setted"),
        CheckResult::ErrorsFound(package) => {
            logger.warn("package.json has some errors tring to correct it");
            do_package_json_corrections(game_data, package, logger)?
        }
    }
    patch_game(game_data, logger)?;
    Ok(())
}
