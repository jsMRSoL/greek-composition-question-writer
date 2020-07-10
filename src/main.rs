use std::process;

fn main() {
    if let Err(e) = string_tweaker::run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
