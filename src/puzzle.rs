use rand::prelude::*;

#[derive(serde::Deserialize)]
pub struct Square {
    pub across: Vec<String>,
    pub down: Vec<String>,
}

impl Square {
    pub fn new(data: &[String]) -> Self {
        let mut down: Vec<String> = vec![];
        for i in 0..5 {
            let mut word = String::new();
            for across in data {
                word += &across.chars().nth(i).unwrap().to_string();
            }
            down.push(word);
        }
        Self {
            across: data.to_vec(),
            down,
        }
    }
}

pub fn load() -> Vec<Vec<String>> {
    let text = std::fs::read_to_string("strings.json").expect("Failure to read strings.json file.");
    serde_json::from_str(&text).expect("Failure to parse strings.json content.")
}

pub fn new(options: &[Vec<String>]) -> Square {
    Square::new(options.choose(&mut rand::rng()).expect("Failure to randomize."))
}