use rpg2linux::{print_error_and_gracefully_exit, processor::Process};

fn main() {
    match Process::new() {
        Ok(process) => match process.execute() {
            Ok(_) => (),
            Err(e) => print_error_and_gracefully_exit(e),
        },
        Err(e) => print_error_and_gracefully_exit(e),
    }
}
