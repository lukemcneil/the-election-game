use rand::seq::SliceRandom;
use rocket_contrib::json;
use reqwest;
use serde::Deserialize;
use serde::Serialize;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use std::env;

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

    pub(crate) fn get_gpt_question(&self, prompt: &str) -> String {
        match send_gpt_request(prompt) {
            Ok(response) => {
                // let response = serde_json::from_value(value)
                let chat_completion: ChatCompletion = serde_json::from_value(response).unwrap();
                chat_completion.choices[0].message.content.clone()
                // serde_json::to_string_pretty(&response).unwrap()
            }
            Err(_err) => {
                "Error".to_string()
            }
        }
    }
}

fn send_gpt_request(prompt: &str) -> Result<serde_json::Value, reqwest::Error> {
    let api_key = env::var("CHAT_GPT_TOKEN").unwrap();
    let gpt_endpoint = "https://api.openai.com/v1/chat/completions"; 

    let content = format!("Make up a question for the game loaded questions about the theme: {prompt}");
    let messages = json!([{
            "role": "user", "content": content 
        }]);

    // Prepare the request payload
    let payload = json!({
        "model": "gpt-3.5-turbo",
        "messages": messages,
        "max_tokens": 150,
        "temperature": 0.7,
    });

    // Create headers with the API key
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Send the HTTP POST request
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(gpt_endpoint)
        .headers(headers)
        .json(&payload)
        .send()?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Parse the JSON response
        let json_response: serde_json::Value = response.json()?;
        Ok(json_response)
    } else {
        // Print the error response if the request was not successful
        let error_response: serde_json::Value = response.json()?;
        Ok(error_response)
        // response.error_for_status()
        // reqwest::Error::
        // Err(reqwest::Error::new(reqwest::StatusCode::from_u16(response.status().as_u16()).unwrap(), format!("{}", error_response)))
    }
}