use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct NWJS {
    pub nwjs_version: String,
    pub especific_nwjs_commands: Vec<String>,
}

// impl NWJS {
//     pub fn new(nwjs_version: String, especific_nwjs_commands: Vec<String>) -> Self {
//         Self {
//             nwjs_version,
//             especific_nwjs_commands,
//         }
//     }
// }

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
