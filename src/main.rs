use rpg_maker_to_linux::{do_stuff, print_error_and_gracefully_exit};

fn main() {
    match do_stuff() {
        Ok(_) => (),
        Err(e) => print_error_and_gracefully_exit(e),
    }
}
