use std::env;

use rpg2linux::{print_error_and_gracefully_exit, errors, processor::{check_directory_and_get_data, check_and_correct_data, process}};

fn main() {
    match do_the_thing() {
        Ok(_) => (),
        Err(e) => print_error_and_gracefully_exit(e),
    }
}

fn do_the_thing() -> Result<(), errors::Errors> {
    let working_directory = env::current_dir()?;
    let mut data = check_directory_and_get_data(working_directory)?;
    check_and_correct_data(&mut data)?;
    process(data)?;
    Ok(())
}

