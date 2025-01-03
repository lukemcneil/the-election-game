use core::str;
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::{collections::BTreeMap, hash::Hash};
use std::{
    collections::{HashMap, HashSet},
    error, fmt,
};

pub(crate) type Result<T> = std::result::Result<T, Error>;
impl<'r> Responder<'r, 'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let body = BadRequest::new(self);
        let body = serde_json::to_string(&body).expect("to BadRequest serialize");
        Ok(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(None, Cursor::new(body))
            .finalize())
    }
}

pub(crate) type Player = String;

#[derive(Serialize, Debug)]
pub(crate) enum Error {
    GameConflict,
    GameNotFound,
    PlayerConflict,
    PlayerNotFound,
    RoundNotInStartState,
    RoundNotInCollectingAnswersState,
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
pub(crate) struct PlayerData {
    /// The player with which the request is associated
    pub(crate) player: Player,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Answer {
    /// The player who gave the answer
    player: Player,
    /// The answer to the question for the round
    pub answer: Player,
}

#[derive(PartialEq)]
pub(crate) enum RoundState {
    Start,
    CollectingAnswers,
    Complete,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Round {
    /// The question for the round
    question: String,
    /// The list of answers given, one per player
    pub(crate) answers: HashSet<Answer>,
}

impl Round {
    fn new(question: String) -> Self {
        Round {
            question,
            answers: HashSet::new(),
        }
    }

    fn state(&self, players: usize) -> RoundState {
        if self.answers.is_empty() {
            RoundState::Start
        } else if self.answers.len() < players {
            RoundState::CollectingAnswers
        } else if self.answers.len() == players {
            RoundState::Complete
        } else {
            panic!("Round in unknown state")
        }
    }

    fn change_question(&mut self, new_question: String) {
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

    pub(crate) fn answer(&mut self, answer: Answer) -> Result<()> {
        let player = &answer.player;
        // Confirm the player exists
        if !self.players.contains(player) {
            return Err(Error::PlayerNotFound);
        }
        if !self.players.contains(&answer.answer) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are collecting answers for the current round
        let state = self.current_round_state();
        if state != RoundState::Start && self.current_round_state() != RoundState::CollectingAnswers
        {
            return Err(Error::RoundNotInCollectingAnswersState);
        }
        let round = self.current_round_mut();
        round.answers.replace(answer);
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
            let mut votes = BTreeMap::new();
            for answer in round.answers {
                *votes.entry(answer.answer).or_insert(0) += 1;
            }
            let max_votes = votes.values().copied().max().unwrap_or(0);

            // Add one to each player with most votes
            votes
                .into_iter()
                .filter(|&(_, count)| count == max_votes)
                .for_each(|(person, _)| *scores.entry(person).or_insert(0) += 1);
        }
        scores
    }

    pub fn change_question(&mut self, new_question: String) {
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
