use std::process::exit;

use errors::Errors;

mod config;
mod errors;
pub mod processor;

const NWJS_URL: &str = "https://dl.nwjs.io";
const NWJS_NORMAL_URL_FORMAT: &str = "{url}/{version}/nwjs-{version}-linux-x64.tar.gz";
const NWJS_SDK_URL_FORMAT: &str = "{url}/{version}/nwjs-sdk-{version}-linux-x64.tar.gz";

pub fn print_error_and_gracefully_exit(error: Errors) -> ! {
    println!("Error happened: {}", error);
    exit(1);
}
