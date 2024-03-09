use std::process;
use rusty::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let configs = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing {}", err);
        process::exit(1);
    });

    if let Err(error_message) = rusty::run(configs){
        println!("Application Error: {error_message}");
        process::exit(1);
    }
}
