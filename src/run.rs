//! # Tsur Run.
//!
//! Runner.
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::ascii::ASCII;
use crate::config::Config;

pub fn run(config: Config) -> Result<HashMap<char, usize>, Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let ascii = ASCII::new();
    if config.count_ascii_characters {
        Ok(ascii.count_ascii_characters(content.as_str()))
    } else {
        panic!("wtf")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_ascii_characters_test_1() {
        let content = "ABC";
        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('A', 1);
        expected.insert('B', 1);
        expected.insert('C', 1);

        let ascii = ASCII::new();
        let result = ascii.count_ascii_characters(content);

        assert_eq!(expected, result);
    }

    #[test]
    fn count_ascii_characters_no_content() {
        // let content = "";
        assert!(true);
    }
}
