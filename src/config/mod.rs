use std::env;

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
    fn config_should_be_initialized() {
        let args = vec![
            "".to_string(),
            "".to_string(),
            ".".to_string()
        ];
        let mut it = args.iter().cloned();
        let conf = Config::new(&mut it).unwrap();

        assert_eq!(Config {
            name: "".to_string(),
            query: "".to_string(),
            filename: ".".to_string(),
            ..conf }, conf);
    }

    #[test]
    #[should_panic]
    fn config_should_panic_without_args() {
        let args = Vec::<String>::new();
        let mut it = args.iter().cloned();
        
        Config::new(&mut it).unwrap();
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub name: String,
    pub query: String,
    pub filename: String,
    pub sensitive: bool,
}

impl Config {
    pub fn new<T>(args: &mut T) -> Result<Config, &'static str>
        where T: Iterator<Item=String>
    {
        let name     = get_or_default(args, "".to_string());
        let query    = get_or_default(args, "".to_string());
        let filename = get_or_default(args, "".to_string());

        let sensitive = env::var("SENSITIVE").is_err();

        if filename == "" {
            Err("Not enough arguments provided")
        } else {
            Ok(Config {name, query, filename, sensitive,})
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