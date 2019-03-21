pub mod config;
pub mod search;

use std::fs;
use std::error::Error;
use config::Config;

pub fn run(conf: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&conf.filename)?;
    let result  = if conf.sensitive {
        search::search(&conf.query, &content)
    } else {
        search::case_insensitive(&conf.query, &content)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}