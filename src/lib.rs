use std::fs;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_value_or_default() {
        let args = vec!["value".to_string()];
        let mut it = args.iter().cloned();

        let val = get_or_default(&mut it, "".to_string());
        assert_eq!(val, "value".to_string());
        let val = get_or_default(&mut it, "value".to_string());
        assert_eq!(val, "value".to_string());
    }

    #[test]
    #[ignore]
    fn config_should_be_initialized_with_and_without_args() {
        let args = vec![
            "".to_string(),
            "".to_string(),
            ".".to_string()
        ];
        let mut it = args.iter().cloned();
        let conf_with_values = Config::new(&mut it).unwrap();
        let conf_with_defaults = Config::new(&mut it).unwrap();

        assert_eq!(conf_with_values, conf_with_defaults);
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub name: String,
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new<T>(args: &mut T) -> Result<Config, &'static str>
        where T: Iterator<Item=String>
    {
        let name     = get_or_default(args, "".to_string());
        let query    = get_or_default(args, "".to_string());
        let filename = get_or_default(args, "".to_string());
        if filename == "" {
            Err("Not enough arguments provided")
        } else {
            Ok(Config {name, query, filename})
        }
    }
}

pub fn get_or_default<T, U>(args: &mut T, default: U) -> U
    where T: Iterator<Item=U>
{
    if let Some(arg) = args.next() {
        arg
    } else {
        default
    }
}

pub fn run(conf: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&conf.filename)?;
    println!("Content \n{}", content);

    Ok(())
}