use std::process::exit;

use errors::Errors;

mod config;
mod errors;
pub mod processor;

pub fn print_error_and_gracefully_exit(error: Errors) -> ! {
    println!("Error happened: {}", error);
    exit(1);
}

pub fn compare_nsjw_versions(ver1: &str, ver2: &str) -> std::cmp::Ordering {
    let components1: Vec<&str> = ver1.split(".").collect();
    let components2: Vec<&str> = ver2.split(".").collect();
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
