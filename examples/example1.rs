extern crate wordsoup;
// use wordsoup::dictionary::*;
use wordsoup::Wordsoup;
use wordsoup::Dictionary;

fn main() {
    let mut d = match Dictionary::from_file("palabras.txt") {
        Err(msg) => panic!(msg),
        Ok(dict) => dict,
    };
    let mut ws: Wordsoup = match Wordsoup::generate_random_wordsoup(10, 10, 20, &mut d) {
        Err(msg) => {
            println!("Error: {}", msg);
            Wordsoup::new(10, 10, Vec::new())
            // panic!("Error");
        }
        Ok(w) => w,
    };
    // dictionary::Dictionary {}
    println!("end of example");
}