use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Result {
    pub count: usize,
    pub questions: Option<crate::puzzle::Square>,
    pub answers: Option<crate::puzzle::Square>,
}
impl Result {
    pub fn default() -> Self {
        Self {
            count: 0,
            questions: None,
            answers: None,
        }
    }
    pub fn complete(&self) -> bool {
        self.count == 10 && self.questions.is_some() && self.answers.is_some()
    }
}

fn make_prompt(prompt: &str, word: &str) -> String {
    prompt.replace("{{word}}", &format!(r#""{word}""#))
}

pub fn run(result: Arc<Mutex<Result>>) {
    let puzzles = crate::puzzle::load();
    let answers = crate::puzzle::new(&puzzles);
    let prompt = std::fs::read_to_string("prompt.txt").expect("Failed to read prompt.txt.");

    let mut count: usize = 0;
    let mut across: Vec<String> = vec![];
    let mut down: Vec<String> = vec![];
    let mut model = llamacpp_embed::start(
        "./llama-model/Qwen3-14B-UD-IQ2_M.gguf",
        "You are a crossword assistant who helps create engaging word clues.",
    );
    for word in &answers.across {
        across.push(llamacpp_embed::chat(&mut model, &make_prompt(&prompt, word)));
        count += 1;
        let mut result_lock = result.lock().unwrap();
        result_lock.count = count;
    }
    for word in &answers.down {
        down.push(llamacpp_embed::chat(&mut model, &make_prompt(&prompt, word)));
        count += 1;
        let mut result_lock = result.lock().unwrap();
        result_lock.count = count;
    }
    llamacpp_embed::stop(&mut model);

    let questions = crate::puzzle::Square::new(&across, &down);

    let mut result_lock = result.lock().unwrap();
    result_lock.questions = Some(questions);
    result_lock.answers = Some(answers);
}
