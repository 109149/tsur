use std::collections::HashMap;

mod ascii;
use tsur::ascii::ascii::ASCII;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_creation() {
        let expected = ASCII::new();
        let result = ASCII::new();

        assert_eq!(expected, result);
    }

    #[test]
    fn count_ascii_characters_in_empty_content() {
        let content = "";

        assert!(ASCII::new().count_ascii_characters(content).is_empty());
    }

    #[test]
    fn count_ascii_characters() {
        let content = "ABC→A";
        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('A', 2);
        expected.insert('B', 1);
        expected.insert('C', 1);
        expected.insert('→', 1);

        let ascii = ASCII::new();
        let result = ascii.count_ascii_characters(content);

        assert_eq!(expected, result);
    }
}
