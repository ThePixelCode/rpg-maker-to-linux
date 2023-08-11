use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{errors::Errors, NWJS_URL};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct FileAsociation {
    pub origin_file: String,
    pub destination_files: Vec<String>,
    pub allows_symlink: bool,
}

// impl FileAsociation {
//     pub fn new(origin_file: String, destination_files: Vec<String>, allows_symlink: bool) -> Self {
//         Self {
//             origin_file,
//             destination_files,
//             allows_symlink,
//         }
//     }
// }

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NWJS {
    pub nwjs_version: String,
    pub especific_nwjs_commands: Vec<String>,
}

impl PartialEq for NWJS {
    fn eq(&self, other: &Self) -> bool {
        self.nwjs_version == other.nwjs_version
    }
}

impl Eq for NWJS {}

impl PartialOrd for NWJS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let components1: Vec<&str> = self.nwjs_version.split(".").collect();
        let components2: Vec<&str> = other.nwjs_version.split(".").collect();
        for (comp1, comp2) in components1.iter().zip(components2.iter()) {
            let num1: u32 = comp1.parse().unwrap_or(0);
            let num2: u32 = comp2.parse().unwrap_or(0);

            match num1.cmp(&num2) {
                std::cmp::Ordering::Equal => continue,
                ord => return Some(ord),
            }
        }

        Some(components1.len().cmp(&components2.len()))
    }
}

impl Ord for NWJS {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let components1: Vec<&str> = self.nwjs_version.split(".").collect();
        let components2: Vec<&str> = other.nwjs_version.split(".").collect();
        for (comp1, comp2) in components1.iter().zip(components2.iter()) {
            let num1: u32 = comp1.parse().unwrap_or(0);
            let num2: u32 = comp2.parse().unwrap_or(0);

            match num1.cmp(&num2) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }

        components1.len().cmp(&components2.len())
    }
}

impl NWJS {
    pub fn get_version(&self) -> Result<String, Errors> {
        let regex = Regex::new(&format!(
            "v{}/",
            self.nwjs_version.replace(".", "\\.").replace("*", "\\d")
        ))?;

        let response = reqwest::blocking::get(NWJS_URL)?;
        let body = response.text()?;

        let mut versions: Vec<String> = regex
            .captures_iter(&body)
            .map(|capture| String::from(&capture[0]).replace("/", ""))
            .collect();

        versions.pop().ok_or(Errors::Unknown)
    }
    // pub fn new(nwjs_version: String, especific_nwjs_commands: Vec<String>) -> Self {
    //     Self {
    //         nwjs_version,
    //         especific_nwjs_commands,
    //     }
    // }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Config {
    // Files needed to link, because windows case insensetive
    pub file_asociations: Vec<FileAsociation>,
    // versions from nwjs where the game was tested
    pub checked_nwjs_versions: Vec<NWJS>,
    // commands needed to execute the game
    pub pre_operation_commands: Vec<String>,
    pub post_operation_commands: Vec<String>,
}

// impl Config {
//     pub fn new(
//         file_asociations: Vec<FileAsociation>,
//         checked_nwjs_versions: Vec<NWJS>,
//         pre_operation_commands: Vec<String>,
//         post_operation_commands: Vec<String>,
//     ) -> Self {
//         Self {
//             file_asociations,
//             checked_nwjs_versions,
//             pre_operation_commands,
//             post_operation_commands,
//         }
//     }
// }
