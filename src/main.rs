use std::env;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_value() {
        let args = vec!["value".to_string()];
        let mut it = args.iter().cloned();
        let val = get_or_default(&mut it, "".to_string());
        assert_eq!(val, "value".to_string());
    }

    #[test]
    fn should_be_default() {
        let args : Vec<String> = Vec::new();
        let mut it = args.iter().cloned();
        let val = get_or_default(&mut it, "value".to_string());
        assert_eq!(val, "value".to_string());
    }
}

#[allow(unused_variables)]
fn main() {
    let mut args = env::args();
    let conf = Config::new(&mut args);

    println!("Looking for: {} in {}", conf.query, conf.filename);
    
    let content = fs::read_to_string(conf.filename)
        .expect("Couldn't read file");

    println!("Content: \n{}", content);
}

#[derive(Debug)]
struct Config {
    name: String,
    query: String,
    filename: String,
}

impl Config {
    fn new<T>(args: &mut T) -> Config 
        where T: Iterator<Item=String>
    {
        let name     = get_or_default(args, "".to_string());
        let query    = get_or_default(args, "".to_string());
        let filename = get_or_default(args, ".".to_string());

        Config {name, query, filename}
    }
}

fn get_or_default<T, U>(args: &mut T, default: U) -> U
    where T: Iterator<Item=U>
{
    if let Some(arg) = args.next() {
        arg
    } else {
        default
    }
}