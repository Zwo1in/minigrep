pub mod config;
pub mod search;

use std::fs;
use std::error::Error;
use config::Config;

pub fn run(conf: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&conf.filename)?;
    for line in search::search(&conf.query, &content) {
        println!("{}", line);
    }
    Ok(())
}