use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Use dbg! to print the args vector for debugging.
    // dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        // Use standard error to print the error message.
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Call run and handle any errors it returns.
    // We don't need the value returned by run, so we only handle the error.
    if let Err(e) = minigrep::run(config) {
        // Use standard error to print the error message.
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
