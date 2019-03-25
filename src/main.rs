use std::env;
use std::process;

use minigrep;
use minigrep::config::Config;

fn main() {
    let mut args = env::args();
    let conf = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("Process exit status: 1");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(&conf) {
        eprintln!("Abborted due to error: {}", e);
        eprintln!("Process exit status: 1");
        process::exit(1);
    }
}