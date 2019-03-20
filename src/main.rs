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
    let name    = get_or_default(&mut args, "".to_string());
    let pattern = get_or_default(&mut args, "".to_string());
    let file    = get_or_default(&mut args, ".".to_string());

    println!("Looking for: {} in {}", pattern, file);
    
    let content = fs::read_to_string(file)
        .expect("Couldn't read file");

    println!("Content: \n{}", content);
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
