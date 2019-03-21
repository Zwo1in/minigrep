use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let mut args = env::args();
    let conf = Config::new(&mut args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(&conf) {
        println!("Abborted due to error: {}", e);
        println!("Process exit status: 1");
        process::exit(1);
    }
}