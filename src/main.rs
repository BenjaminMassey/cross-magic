mod llm;
mod puzzle;

fn main() {
    let puzzles = puzzle::load();
    let answers = puzzle::new(&puzzles);
    let client = reqwest::blocking::Client::new();
    let prompt = std::fs::read_to_string("prompt.txt").expect("Failed to read prompt.txt.");

    let mut across: Vec<String> = vec![];
    let mut down: Vec<String> = vec![];
    for word in &answers.across {
        across.push(llm::chat(&client, &(prompt.clone() + &word)));
    }
    for word in &answers.down {
        down.push(llm::chat(&client, &(prompt.clone() + &word)));
    }
    let questions = puzzle::Square::new(&across, &down);
    
    println!("\n==============\n=== ACROSS ===\n==============\n");
    for i in 0..5 {
        println!("{}\n==>\n{}\n", questions.across[i], answers.across[i]);
    }
    println!("\n============\n=== DOWN ===\n============\n");
    for i in 0..5 {
        println!("{}\n==>\n{}\n", questions.down[i], answers.down[i]);
    }
}
