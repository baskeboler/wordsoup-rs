extern crate rand;
use std::vec::*;
use std::iter::*;
use rand::*;
use rand::distributions::*;
use std::collections::HashMap;
use std::collections::hash_map::*; //HashMap;
mod dictionary;
use dictionary::*;
pub type Dictionary = dictionary::Dictionary;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    use super::*;
    #[test]
    fn test_generate() {
        let mut d = match Dictionary::from_file("palabras.txt") {
            Err(msg) => panic!(msg),
            Ok(dict) => dict,
        };
        let mut ws: Wordsoup = match Wordsoup::generate_random_wordsoup(10, 10, 20, &mut d) {
            Err(msg) => {
                println!("Error: {}", msg);
                panic!("Error");
            }
            Ok(w) => w,
        };
        println!("{:?}", ws);

        println!("{}", ws.to_string());
    }
}

static ASCII_LOWER: [char; 26] = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
];


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Orientation {
    VERTICAL,
    HORIZONTAL,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Word {
    letters: String,
    orientation: Orientation,
    pos: Position,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cell {
    letter: u8,
    pos: Position,
}

#[derive(Debug, Clone)]
pub struct Wordsoup {
    w: i32,
    h: i32,
    words: Vec<Word>,
}

impl Word {
    pub fn new(letters: &str, orientation: Orientation, position: Position) -> Word {
        Word {
            letters: String::from(letters),
            orientation: orientation,
            pos: position,
        }
    }
    pub fn cells(&self) -> Vec<Cell> {
        let mut res: Vec<Cell> = Vec::new();
        let a = self.letters.char_indices();
        for (id, c) in a {
            let mut p = self.pos;
            if self.orientation == Orientation::VERTICAL {
                p.y = p.y + id as i32
            }
            if self.orientation == Orientation::HORIZONTAL {
                p.x = p.x + id as i32
            }
            let cell = Cell {
                letter: c as u8,
                pos: p,
            };
            res.push(cell);
        }
        res
    }

    pub fn conflicts(&self, other: &Word) -> bool {

        for my_cell in self.cells() {
            for other_cell in other.cells() {
                if my_cell.pos == other_cell.pos && my_cell.letter != other_cell.letter {
                    return true;
                }
            }
        }
        false
    }
}

impl Wordsoup {
    pub fn new(w: i32, h: i32, words: Vec<Word>) -> Wordsoup {
        Wordsoup {
            w: w,
            h: h,
            words: Vec::from(words),
        }
    }
    pub fn add_word(&mut self, word: Word) {
        self.words.push(word);
    }
    pub fn fits(&self, word: &Word) -> bool {
        if word.orientation == Orientation::VERTICAL {
            if word.pos.y + word.letters.len() as i32 > self.h {
                return false;
            }
        } else {
            if word.pos.x + word.letters.len() as i32 > self.w {
                return false;
            }
        }
        true
    }

    pub fn conflicts(&self, word: &Word) -> bool {
        self.words.iter().any(|w: &Word| w.conflicts(word))
    }
    pub fn addWord(&mut self, word: &str) -> Result<Word, &str> {
        if word.len() as i32 > self.h && word.len() as i32 > self.w {
            return Err("Word doesnt fit");
        }
        let mut r = rand::chacha::ChaChaRng::new_unseeded();
        let mut rng = rand::thread_rng();
        let rH = rand::distributions::Range::new(1, self.h);
        let rW = rand::distributions::Range::new(1, self.w);
        // let ors = rand::distributions::Range::new(0u8, 2u8);
        // rH.ind_sample(r);
        let orients = [Orientation::HORIZONTAL, Orientation::VERTICAL];
        for i in 0..POS_RETRIES {
            let p = Position {
                x: rW.ind_sample(&mut r),
                y: rH.ind_sample(&mut r),
            };
            let orient = rng.choose(&orients).unwrap();
            let w = Word::new(word, orient.clone(), p);
            if self.fits(&w) && !self.conflicts(&w) {
                let copy = w.clone();
                self.add_word(w);
                return Ok(copy);
            }
        }
        Err("Retries exceeded")
    }

    pub fn render(&self) -> Result<HashMap<i32, u8>, &str> {
        let mut res: HashMap<i32, u8> = HashMap::new();
        let i = self.words.as_slice().iter();
        i.for_each(|w: &Word| {
            w.cells().as_slice().iter().for_each(|c: &Cell| {

                res.insert(c.pos.y * self.w + c.pos.x, c.letter);
            });
        });
        for index in 0..self.w * self.h {
            if !res.contains_key(&index) {
                res.insert(index, self.randomLetter());
            }
        }
        Ok(res)
    }
    fn randomLetter(&self) -> u8 {
        *rand::thread_rng().choose(&ASCII_LOWER).unwrap() as u8
    }

    pub fn to_string(&self) -> String {
        let mut res: String = String::new();
        let m = self.render().unwrap();
        for i in 0..self.w * self.h {
            if i % self.w == 0 {
                res += "\n";
            }
            let character = *m.get(&i).unwrap() as char;
            res.push(character);
        }
        res
    }

    pub fn generate_random_wordsoup(
        height: i32,
        width: i32,
        n: i32,
        dict: &mut dictionary::Dictionary,
    ) -> Result<Wordsoup, &str> {
        if height < 1 || width < 1 {
            return Err("invalid dimensions");
        }

        let mut ws = Wordsoup::new(width, height, Vec::new());

        'outer: for i in 0..n {
            'inner: for j in 0..ADD_RETRIES {
                let word = dict.rand_word().unwrap();
                match ws.addWord(word) {
                    Err(_) => {
                        println!("could not add word: {}", word);
                        continue 'inner;
                    }
                    Ok(w) => {
                        println!("added {:?}", w);
                        continue 'outer;
                    }
                }
            }
            println!("{:?}", ws);
            return Err("could not generate");
        }
        Ok(ws)
    }
}
const ADD_RETRIES: i32 = 10;
const POS_RETRIES: i32 = 1000;