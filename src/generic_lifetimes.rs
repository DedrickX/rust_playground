extern crate rand;

fn simulate_game<'a>(home: &'a str, visitors: &'a str) -> &'a str {
    if rand::random() {
        home
    } else {
        visitors
    }
}

pub fn game() {
    let home = String::from("Predmier");
    let visitors = String::from("Žilina");
    let winner = simulate_game(&home, &visitors);
    println!("Game {} vs {} winner: {}", home, visitors, winner);
}

struct Stemmer {
    suffix: String,
}

impl Stemmer {
    fn stem<'a>(&self, word: &'a str) -> &'a str {
        if word.ends_with(&self.suffix) {
            let index = word.rfind(&self.suffix).expect("Should be found!");
            &word[0..index]
        } else {
            word
        }
    }
}

pub fn stemming_words() {
    let word = String::from("Scammer");
    let word_stemmer = Stemmer {
        suffix: String::from("mer"),
    };
    let result = word_stemmer.stem(&word);
    println!("Word {} is stemmed to {}", word, result);
}
