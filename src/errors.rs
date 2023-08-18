use std::{io, string};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("User has cancelled the operation")]
    UserCancelled,
    #[error("Cannot recognize {0}, {1}")]
    UnknownFolder(String, &'static str),
    #[error("IO Error {0}")]
    IOError(#[from] io::Error),
    #[error("Invalid Config File")]
    InvalidFile(#[from] string::FromUtf8Error),
    #[error("Invalid json format")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Error during {0} fase, the error was:\n{1}")]
    ProcessError(&'static str, String),
    #[error("Missing NWJS version")]
    MissingNWJSVersions,
    #[error("Missing file association")]
    MissingFileAssociations,
    #[error("Invalid NWJS version")]
    InvalidNWJSVersion(#[from] regex::Error),
    #[error("Connection Error")]
    ConnectionError(#[from] reqwest::Error),
    #[error("Unknown error")]
    Unknown,
}
