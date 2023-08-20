use core::num;
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
    #[error("Invalid Config File: {0}")]
    InvalidFile(#[from] string::FromUtf8Error),
    #[error("Invalid json format: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Error during {0} fase, the error was:\n{1}")]
    ProcessError(&'static str, String),
    #[error("Missing NWJS version")]
    MissingNWJSVersions,
    #[error("Missing file association")]
    MissingFileAssociations,
    #[error("Invalid NWJS version: {0}")]
    InvalidNWJSVersion(#[from] regex::Error),
    #[error("Connection Error: {0}")]
    ConnectionError(#[from] reqwest::Error),
    #[error("Error during parsing input: {0}")]
    ParseError(#[from] num::ParseIntError),
    #[error("Unknown error")]
    Unknown,
}
