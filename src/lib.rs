use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    filename: String,
    count_ascii_characters: bool,
}

impl Config {
    pub fn new(args: clap::ArgMatches) -> Result<Config, &'static str> {
        let count_ascii_characters = match args.value_of("mode") {
            Some(v) => {
                if v == "count chars" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        // let for_fun = args.is_present("bool_flag");

        let filename = match args.value_of("file") {
            Some(f) => f.to_string(),
            None => return Err("No filename"),
        };

        Ok(Config {
            filename,
            count_ascii_characters,
        })
    }
}

#[derive(Debug)]
pub struct ASCII {
    ascii: HashMap<char, usize>,
}

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
        self.ascii
    }

    pub fn get_ascii(self) -> HashMap<char, usize> {
        self.ascii
    }
}

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
