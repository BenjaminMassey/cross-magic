// https://github.com/ollama/ollama/blob/main/docs/api.md

#[derive(serde::Serialize, serde::Deserialize)]
struct Request {
    model: String,
    prompt: String,
    stream: bool,
    think: bool,
}
impl Request {
    fn new(model: &str, prompt: &str) -> Self {
        Self {
            model: model.to_owned(),
            prompt: prompt.to_owned(),
            stream: false,
            think: false, // TODO: maybe wanted, but then need parsing
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Response {
    response: Option<String>,
}

pub fn chat(model: &str, client: &reqwest::blocking::Client, prompt: &str) -> String {
    let body = serde_json::to_string(&Request::new(model, prompt)).expect("JSON to error");
    let url = "http://localhost:11434/api/generate"; // TODO: setting
    let result = client
        .post(url)
        .body(body)
        .send()
        .expect("LLM endpoint error");
    let text = result.text().expect("LLM text error");
    let response: Response = serde_json::from_str(&text).expect("JSON from error");
    let text = response.response.expect("Broken response");
    sanitize(&text)
}

fn sanitize(original: &str) -> String {
    let replaces = vec![
        ("’", "'"),
        ("“", "\""),
        ("”", "\""),
        ("\n", " "),
        ("\r", " "),
        ("—", " - "),
    ];
    let mut new = original.to_owned();
    for (o, n) in &replaces {
        new = new.replace(o, n);
    }
    new
}
