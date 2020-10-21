//! # Tsur ASCII structure.
//!
//! ASCII structure saves number of (counted) characters inside hashmap.
use std::collections::HashMap;
// use std::ops::Deref;

#[derive(Debug, PartialEq, Eq)]
pub struct ASCII {
    ascii: HashMap<char, usize>,
}

// impl Deref for ASCII {
//     type Target = HashMap<char, usize>;

//     fn deref(&self) -> &Self::Target {
//         &self.ascii
//     }
// }

#[allow(dead_code)]
impl ASCII {
    /// Creates new instance of ASCII structure.
    ///
    /// # Examples
    ///
    /// ```
    /// # pub use tsur::ascii::ascii::ASCII;
    /// let ascii = ASCII::new();
    /// ```
    pub fn new() -> ASCII {
        ASCII {
            ascii: HashMap::new(),
        }
    }

    /// Returns hashmap of how many times each character occur in a content.
    ///
    /// # Arguments
    ///
    /// * `content` - A string slice in which we want to count character occurrences
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # pub use tsur::ascii::ascii::ASCII;
    /// # use std::collections::HashMap;
    /// let content = "ABA→";
    /// let mut expected: HashMap<char, usize> = HashMap::new();
    /// expected.insert('A', 2);
    /// expected.insert('B', 1);
    /// expected.insert('→', 1);
    ///
    /// let ascii = ASCII::new();
    /// let result = ascii.count_ascii_characters(content);
    ///
    /// assert_eq!(expected, result);
    /// ```
    pub fn count_ascii_characters(mut self, content: &str) -> HashMap<char, usize> {
        let _ = content
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| {
                self.ascii.entry(c).and_modify(|v| *v += 1).or_insert(1);
                c
            })
            .collect::<Vec<_>>();
        self.get_ascii()
    }

    /// Returns result hashmap character occurrences.
    pub fn get_ascii(self) -> HashMap<char, usize> {
        self.ascii
    }
}
