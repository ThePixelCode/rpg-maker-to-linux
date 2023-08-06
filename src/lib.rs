use std::process::exit;

use errors::Errors;

mod config;
mod errors;
pub mod processor;

pub fn print_error_and_gracefully_exit(error: Errors) -> ! {
    println!("Error happened: {}", error);
    exit(1);
}
