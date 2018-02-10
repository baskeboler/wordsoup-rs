use std::vec::*;
use std::iter::*;
use rand;
use rand::Rng;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::convert::*;
use rand::seq::*;
use std::str;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dict() {
        let d = match Dictionary::from_file("palabras.txt") {
            Err(msg) => panic!(msg),
            Ok(dict) => dict,
        };
        assert_eq!(d.words.len(), 10897);
        println!("{:?}", d.rand_word());
    }

    #[test]
    fn test_words() {
        let mut d = match Dictionary::from_file("palabras.txt") {
            Err(msg) => panic!(msg),
            Ok(dict) => dict,
        };
        let words = d.rand_words(15);
        assert_eq!(words.len(), 15);

        println!("{:?}", words);
    }
}

#[derive(Debug)]
pub struct Dictionary {
    words: Vec<String>,
}



impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary { words: Vec::new() }
    }

    pub fn from_words(words: Vec<String>) -> Dictionary {
        let REPLACEMENTS: HashMap<&str, &str> = [
            ("á", "a"),
            ("é", "e"),
            ("í", "i"),
            ("ó", "o"),
            ("ú", "u"),
        ].iter()
            .cloned()
            .collect();
        let words2 = words
            .iter()
            .cloned()
            .map(|s: String| {
                let mut s2 = s.to_lowercase();
                for k in REPLACEMENTS.keys() {
                    s2 = s2.replace(k, REPLACEMENTS.get(k).unwrap());
                }
                s2
            })
            .collect();

        Dictionary { words: words2 }
    }

    pub fn from_file(file_name: &str) -> Result<Dictionary, &str> {
        let mut f = match File::open(file_name) {
            Err(_) => return Err("Could not open file"),
            Ok(file) => file,
        };

        let mut buffer = String::new();
        match f.read_to_string(&mut buffer) {
            Err(_) => return Err("Could not process dictionary file"),
            _ => (),
        }
        let mut lines = buffer.lines();
        let vec: Vec<&str> = lines.collect::<Vec<&str>>();
        let strVec = vec.iter().map(|s| String::from(*s)).collect();
        Ok(Dictionary::from_words(strVec))
    }

    pub fn rand_word(&self) -> Option<&String> {
        rand::thread_rng().choose(&self.words)
    }

    pub fn rand_words(&mut self, n: i32) -> Vec<String> {

        let mut rng = rand::thread_rng();

        sample_slice(&mut rng, self.words.as_mut_slice(), n as usize)
    }
}