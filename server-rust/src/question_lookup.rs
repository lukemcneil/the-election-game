use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const DEFAULT_QUESTION: &str = "Answer the question you would have liked to be asked?";

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    finish_reason: String,
    index: u32,
    logprobs: Option<serde_json::Value>,
    message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    content: String,
    role: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChatCompletion {
    choices: Vec<Choice>,
    created: u64,
    id: String,
    model: String,
    object: String,
    system_fingerprint: Option<serde_json::Value>,
    usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}
#[derive(Default)]
pub(crate) struct QuestionLookup {
    questions: Vec<String>,
}

impl QuestionLookup {
    pub(crate) fn populate_from_file(&mut self, path: &Path) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.questions.push(line?);
        }
        Ok(())
    }

    pub(crate) fn get(&self) -> String {
        let mut rng = rand::thread_rng();
        self.questions
            .choose(&mut rng)
            .map_or_else(|| String::from(DEFAULT_QUESTION), |q| q.clone())
    }
}
