pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .into_iter()
        .filter(|line| {
            line.contains(query)
        })
        .map(|it| it.trim())
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();

    content.lines()
        .into_iter()
        .filter(|line| {
            line.to_lowercase()
                .contains(query)
        })
        .map(|it| it.trim())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "productive";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let right = search(query, content);

        let left = vec!["safe, fast, productive."];

        assert_eq!(right, left)
    }

    #[test]
    fn two_results() {
        let query = "public";
        let content =
            "public class Main {
            public static void main(String... args) {
                System.out.println('Hello, World!');
            }
        }";

        let left = search(query, content);
        let right = vec!["public class Main {", "public static void main(String... args) {"];

        assert_eq!(left, right)
    }

    #[test]
    fn case_sensitive() {
        let query = "Public";
        let content =
            "public class Main {
            public static void main(String... args) {
                System.out.println('Hello, World!');
            }
        }";

        let left = search(query, content);
        let right: Vec<&str> = vec![];

        assert_eq!(left, right)
    }

    #[test]
    fn case_insensitive() {
        let query = "Public";
        let content =
            "public class Main {
            public static void main(String... args) {
                System.out.println('Hello, World!');
            }
        }";

        let left = search_case_insensitive(query, content);
        let right: Vec<&str> = vec!["public class Main {", "public static void main(String... args) {"];

        assert_eq!(left, right)
    }
}