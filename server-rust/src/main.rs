#[macro_use]
extern crate rocket;

mod question_lookup;
mod types;

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use question_lookup::QuestionLookup;
use rocket::config::LogLevel;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::{Config, State};
use rocket_cors::{AllowedOrigins, CorsOptions};
use structopt::StructOpt;
use types::{Answer, Game, Player, PlayerData, Result};

type Games = Mutex<types::Games>;
type Questions = Mutex<QuestionLookup>;

#[get("/heartbeat")]
fn heartbeat() -> &'static str {
    "heartbeat"
}

#[put("/game/<game_id>", data = "<player>")]
fn create_game(
    game_id: &str,
    player: Json<PlayerData>,
    games: &State<Arc<Games>>,
    questions: &State<Arc<Questions>>,
) -> Result<()> {
    let mut games = games.lock().unwrap();
    let player = player.into_inner();
    games.create(
        game_id.to_string(),
        player.player,
        questions.lock().unwrap().get(),
    )
}

#[post("/game/<game_id>", data = "<player>")]
fn join_game(game_id: &str, player: Json<PlayerData>, games: &State<Arc<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    let player = player.into_inner();
    game.add_player(player.player)
}

#[get("/game/<game_id>")]
fn game(game_id: &str, games: &State<Arc<Games>>) -> Result<Json<Game>> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    // TODO: This clone is ugly
    Ok(Json(game.clone()))
}

#[post("/game/<game_id>/answer", data = "<answer>")]
fn answer(
    game_id: &str,
    answer: Json<Answer>,
    games: &State<Arc<Games>>,
    questions: &State<Arc<Questions>>,
) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    let answer = answer.into_inner();
    game.answer(answer)?;
    game.add_round_if_complete(questions.lock().unwrap().get());
    Ok(())
}

#[delete("/game/<game_id>/exit", data = "<player>")]
fn exit_game(game_id: &str, player: Json<PlayerData>, games: &State<Arc<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    let player = player.into_inner();
    game.remove_player(player.player)
}

#[delete("/game/<game_id>")]
fn delete_game(game_id: &str, games: &State<Arc<Games>>) {
    let mut games = games.lock().unwrap();
    games.delete(game_id)
}

#[get("/game/<game_id>/score")]
fn get_score(game_id: &str, games: &State<Arc<Games>>) -> Result<Json<HashMap<Player, i32>>> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?.clone();
    Ok(Json(game.get_score()))
}

#[post("/game/<game_id>/change_question")]
fn change_question(
    game_id: &str,
    games: &State<Arc<Games>>,
    questions: &State<Arc<Questions>>,
) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    game.change_question(questions.lock().unwrap().get());
    Ok(())
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// The path to a file containing newline delimited questions.
    #[structopt(long = "questions-file")]
    questions_file: Option<PathBuf>,
    /// An IP address the application will listen on.
    #[structopt(long = "host", short = "H", default_value = "0.0.0.0")]
    address: IpAddr,
    /// A port number to listen on.
    #[structopt(long = "port", short = "P", default_value = "8172")]
    port: u16,
    /// The log level.
    #[structopt(
        default_value = "normal",
        long = "log-level",
        possible_values = &["off", "debug", "normal", "critical"]
    )]
    log_level: LogLevel,
}

#[launch]
fn rocket() -> _ {
    let opt = Opt::from_args();
    let mut questions = QuestionLookup::default();
    if let Some(questions_file) = opt.questions_file {
        // Populate the questions
        if let Err(e) = questions.populate_from_file(&questions_file) {
            eprintln!(
                "Failed to populate questions from file {:?}, err: {}",
                questions_file, e
            );
            std::process::exit(1);
        }
    }

    let config = Config {
        address: opt.address,
        port: opt.port,
        log_level: opt.log_level,
        ..Config::default()
    };
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Put,
                Method::Delete,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);
    rocket::build()
        .configure(config)
        .attach(cors.to_cors().unwrap())
        .mount(
            "/api/v1",
            routes![
                heartbeat,
                create_game,
                join_game,
                game,
                answer,
                exit_game,
                delete_game,
                get_score,
                change_question,
            ],
        )
        .manage(Arc::new(Games::default()))
        .manage(Arc::new(Mutex::new(questions)))
}
