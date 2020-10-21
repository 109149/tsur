//! # Tsur ASCII.
//!
//! ASCII structure.
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
    pub fn new() -> ASCII {
        ASCII {
            ascii: HashMap::new(),
        }
    }

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

    pub fn get_ascii(self) -> HashMap<char, usize> {
        self.ascii
    }
}
