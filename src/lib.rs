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

    #[test]
    fn search_should_return_whole_line() {
        let query = "arg";
        let content = "\
He wasn't right about it
nor his argumentation was
but he was still talkin'";

        assert_eq!(vec!["nor his argumentation was"], search(&query, &content));
    }

    #[test]
    fn search_should_return_empty() {
        let query = "tą";
        let content = "\
He wasn't right about it
nor his argumentation was
but he was still talkin'";

        assert_eq!(Vec::<&str>::new(), search(&query, &content));
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
    for line in search(&conf.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::<&str>::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}