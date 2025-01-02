mod traits;

use base64::prelude::*;
use core::str;
use rocket::{data, response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::hash::Hash;
use std::io::prelude::*;
#[cfg(test)]
use std::iter::FromIterator;
use std::{
    collections::{HashMap, HashSet},
    error, fmt,
};
use std::{fs::File, path::Path};

pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type Player = String;
pub(crate) type GameId = String;
pub(crate) type Prompt = String;

#[derive(Serialize, Debug, Clone)]
pub(crate) enum GameMode {
    Text,
    Pictures,
}

#[derive(Serialize, Debug)]
pub(crate) enum Error {
    GameConflict,
    GameNotFound,
    PlayerConflict,
    PlayerNotFound,
    RoundNotInStartState,
    RoundNotInCollectingAnswersState,
    RoundNotInCollectingGuessesState,
    GuessedPlayerNotFound,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImageCompletion {
    images: Vec<String>,
    parameters: serde_json::Value,
    info: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameConflict => write!(f, "game conflict"),
            Self::GameNotFound => write!(f, "game not found"),
            Self::PlayerConflict => write!(f, "player conflict"),
            Self::PlayerNotFound => write!(f, "player not found"),
            Self::RoundNotInStartState => write!(f, "round not in start state"),
            Self::RoundNotInCollectingAnswersState => {
                write!(f, "round not in collecting answer state")
            }
            Self::RoundNotInCollectingGuessesState => {
                write!(f, "round not in collecting guess state")
            }
            Self::GuessedPlayerNotFound => write!(f, "guessed player not found"),
        }
    }
}

impl error::Error for Error {}

#[derive(Deserialize, Serialize)]
pub(crate) struct BadRequest {
    error: String,
    message: String,
}

impl BadRequest {
    fn new(error: Error) -> Self {
        Self {
            error: format!("{:?}", error),
            message: format!("{}", error),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PromptData {
    pub(crate) prompt: Prompt,
}

#[cfg(test)]
impl PromptData {
    pub(crate) fn new(prompt: &str) -> Self {
        Self {
            prompt: Prompt::from(prompt),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PlayerData {
    /// The player with which the request is associated
    pub(crate) player: Player,
}

#[cfg(test)]
impl PlayerData {
    pub(crate) fn new(player: &str) -> Self {
        Self {
            player: Player::from(player),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Answer {
    /// The player who gave the answer
    player: Player,
    /// The answer to the question for the round
    pub answer: String,
}

#[cfg(test)]
impl Answer {
    pub(crate) fn new(player: &str, answer: &str) -> Self {
        Self {
            player: Player::from(player),
            answer: String::from(answer),
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Picture {
    pub prompt: String,
    pub url: String,
}

impl Picture {
    pub(crate) fn new(prompt: &str, input_url: &str) -> Self {
        Self {
            prompt: String::from(prompt),
            url: String::from(input_url),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Guess {
    /// The player making the guess
    pub player: Player,
    /// The list of guessed answers, one per player
    pub answers: HashSet<Answer>,
}

#[cfg(test)]
impl Guess {
    pub(crate) fn new(player: &str, guess: Vec<Answer>) -> Self {
        Self {
            player: Player::from(player),
            answers: HashSet::from_iter(guess),
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum RoundState {
    Start,
    CollectingAnswers,
    CollectingGuesses,
    Complete,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Round {
    /// The question for the round
    question: String,
    /// The list of answers given, one per player
    pub(crate) answers: HashSet<Answer>,
    /// The list of guesses made, one per player
    pub(crate) guesses: HashSet<Guess>,
    pub(crate) pictures: HashSet<Picture>,
}

impl Round {
    fn new(question: String) -> Self {
        Round {
            question,
            answers: HashSet::new(),
            guesses: HashSet::new(),
            pictures: HashSet::new(),
        }
    }

    fn state(&self, players: usize) -> RoundState {
        if self.answers.is_empty() {
            RoundState::Start
        } else if self.answers.len() < players {
            RoundState::CollectingAnswers
        } else if self.guesses.len() < players {
            RoundState::CollectingGuesses
        } else if self.answers.len() == players && self.guesses.len() == players {
            RoundState::Complete
        } else {
            panic!("Round in unknown state")
        }
    }

    fn change_question(&mut self, new_question: String) -> () {
        self.question = new_question;
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Game {
    /// The list of players in the game
    pub(crate) players: HashSet<String>,
    /// The list of rounds in the game with the most recent round being the last item in the list
    pub(crate) rounds: Vec<Round>,
}

fn send_stable_diffusion_request(
    prompt: &str,
) -> std::result::Result<serde_json::Value, reqwest::Error> {
    let stable_diffusion_endpoint =
        "https://a627-2600-1700-5b80-980-54ec-6d29-2cd7-fa0b.ngrok-free.app/sdapi/v1/txt2img";

    // Prepare the request payload
    let payload = json!({
        "prompt": prompt,
        "steps": 8
    });

    // Send the HTTP POST request
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(stable_diffusion_endpoint)
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

impl Game {
    pub(crate) fn add_player(&mut self, player: Player) -> Result<()> {
        // Only allow adding players at the start of a round
        if self.current_round_state() != RoundState::Start {
            return Err(Error::RoundNotInStartState);
        }
        if self.players.insert(player) {
            Ok(())
        } else {
            Err(Error::PlayerConflict)
        }
    }

    pub(crate) fn remove_player(&mut self, player: Player) -> Result<()> {
        // Only allow removing players at the start of a round
        if self.current_round_state() != RoundState::Start {
            return Err(Error::RoundNotInStartState);
        }
        self.players.remove(&player);
        Ok(())
    }

    pub(crate) fn answer(&mut self, answer: Answer, game_id: String) -> Result<()> {
        let player = &answer.player;
        // Confirm the player exists
        if !self.players.contains(player) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are collecting answers for the current round
        let state = self.current_round_state();
        if state != RoundState::Start && self.current_round_state() != RoundState::CollectingAnswers
        {
            return Err(Error::RoundNotInCollectingAnswersState);
        }

        let prompt = &answer.answer;
        let mut data_file;
        match send_stable_diffusion_request(prompt) {
            Ok(response) => {
                let chat_completion: ImageCompletion = serde_json::from_value(response).unwrap();
                let x = &chat_completion.images[0];
                let img_buffer = base64::decode(x).unwrap();
                data_file = File::create(Path::new(&format!("pictures/{}{}.png", game_id, player)))
                    .expect("creation failed");
                data_file.write(&img_buffer[0..]).expect("write failed");
            }
            Err(_err) => {}
        }
        let input_url = &format!("pictures/{}{}.png", game_id, player);
        let new_picture = Picture {
            prompt: prompt.to_string(),
            url: input_url.to_string(),
        };
        // let new_answer = Answer::new("ayden", data_file)
        let round = self.current_round_mut();
        round.pictures.replace(new_picture);
        round.answers.replace(answer);
        Ok(())
    }

    pub(crate) fn guess(&mut self, guess: Guess) -> Result<()> {
        let player = &guess.player;
        // Confirm the player exists
        if !self.players.contains(player) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are adding collecting for the current round
        if self.current_round_state() != RoundState::CollectingGuesses {
            return Err(Error::RoundNotInCollectingGuessesState);
        }
        // Confirm the guesses are valid
        for g in &guess.answers {
            if !self.players.contains(&g.player) {
                return Err(Error::GuessedPlayerNotFound);
            }
        }
        // Add or replace the guess
        let round = self.current_round_mut();
        round.guesses.replace(guess);
        Ok(())
    }

    pub(crate) fn add_round_if_complete(&mut self, question: String) {
        if self.current_round_state() == RoundState::Complete {
            self.add_round(question);
        }
    }

    fn add_round(&mut self, question: String) {
        self.rounds.push(Round::new(question));
    }

    pub(crate) fn current_round(&self) -> &Round {
        let index = self.rounds.len() - 1;
        &self.rounds[index]
    }

    fn current_round_mut(&mut self) -> &mut Round {
        let index = self.rounds.len() - 1;
        &mut self.rounds[index]
    }

    fn current_round_state(&self) -> RoundState {
        let players = self.players.len();
        let round = self.current_round();
        round.state(players)
    }

    pub fn get_score(&self) -> HashMap<String, i32> {
        let mut scores = HashMap::new();
        let game = self.clone();
        for round in game.rounds {
            for guess in round.guesses {
                for answer in guess.answers {
                    let score = scores.entry(guess.player.clone()).or_insert(0);
                    if round.answers.contains(&answer) {
                        *score += 1;
                    } else {
                        *score -= 1;
                    }
                }
            }
        }
        scores
    }

    pub fn change_question(&mut self, new_question: String) -> () {
        self.rounds
            .last_mut()
            .unwrap()
            .change_question(new_question);
    }
}

#[derive(Default)]
pub(crate) struct Games(HashMap<String, Game>);

impl Games {
    #[allow(clippy::map_entry)]
    pub(crate) fn create(
        &mut self,
        game_id: String,
        initial_player: Player,
        initial_question: String,
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            let mut game = Game::default();
            game.add_round(initial_question);
            game.add_player(initial_player)?;
            self.0.insert(game_id, game);
            Ok(())
        }
    }

    pub(crate) fn get(&mut self, game_id: &str) -> Result<&mut Game> {
        self.0.get_mut(game_id).ok_or(Error::GameNotFound)
    }

    pub(crate) fn delete(&mut self, game_id: &str) {
        self.0.remove(game_id);
    }
}
