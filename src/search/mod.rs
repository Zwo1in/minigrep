#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn case_insensitive_should_match() {
        let query = "hE";
        let content = "\
he is
SHE was
...";

        assert_eq!(vec!["he is", "SHE was"], case_insensitive(&query, &content));
    }
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

pub fn case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::<&str>::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}