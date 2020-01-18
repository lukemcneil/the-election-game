const express = require("express");
const app = express();
const port = 3000;
var cors = require("cors");
app.use(express.json());
const util = require("util");
app.use(cors());

// create game
app.put(
  "/api/v1/game/:id",
  (req: { params: { id: number }; body: Player }, res: any) => {
    let check: boolean = false;
    games.map((game: Game) => {
      if (game.id == req.params.id) {
        res.status(409);
        console.log("A game with the specified game id already exists");
        check = true;
      }
    });
    if (!check) {
      let madeGame = makeGame(req.params.id);
      console.log(req.body);
      addPlayerToGame(req.body.player, madeGame.id);
      console.log(util.inspect(madeGame, { depth: null }));
      res.status(201);
      console.log("Successfully created the game");
    }
    res.send();
  }
);

// add player
app.post(
  "/api/v1/game/:id",
  (req: { params: { id: number }; body: Player }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
      return game.id == req.params.id;
    });
    if (!matchingGame) {
      res.status(404);
      console.log("A game with the specified game id does not exist");
      res.send();
      return;
    }
    let matchingPlayer: String | undefined = matchingGame.players.find(
      (player: String) => {
        return player == req.body.player;
      }
    );
    if (!matchingPlayer) {
      addPlayerToGame(req.body.player, req.params.id);
      res.status(200);
      console.log("Successfully joined the game");
      console.log(util.inspect(matchingGame, { depth: null }));
    } else {
      res.status(409);
      console.log("A player with the specified game id already exists");
    }
    res.send();
  }
);

// get game data
app.get("/api/v1/game/:id", (req: { params: { id: number } }, res: any) => {
  let matchingGame: Game | undefined = games.find((game: Game) => {
    return game.id == req.params.id;
  });
  if (!matchingGame) {
    res.status(404);
    console.log("A game with the specified game id does not exist");
    res.send();
    return;
  }
  console.log(util.inspect(matchingGame, { depth: null }));
  res.status(200);
  let result: Game = {
    id: matchingGame.id,
    phase: matchingGame.phase,
    players: matchingGame.players,
    rounds: matchingGame.rounds
  };
  delete result.id;
  delete result.phase;
  res.json(result);
});

//  submit answer
app.post(
  "/api/v1/game/:id/answer",
  (req: { params: { id: number }; body: Answer }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
      return game.id == req.params.id;
    });
    if (!matchingGame) {
      res.status(404);
      console.log(
        "A game with the specified game id does not exist or the specified player is not part of the game"
      );
      res.send();
      return;
    }
    if (matchingGame.phase != "answer") {
      console.log("not in answering phase");
      res.status(404);
      res.send();
      return;
    }
    let foundPlayer: String | undefined = matchingGame.players.find(
      (player: String) => {
        return player == req.body.player;
      }
    );
    if (!foundPlayer) {
      res.status(404);
      console.log(
        "A game with the specified game id does not exist or the specified player is not part of the game"
      );
      res.send();
      return;
    }
    let previousAnswer: Answer | undefined = matchingGame.rounds[
      matchingGame.rounds.length - 1
    ].answers.find((answer: Answer) => {
      return answer.player == req.body.player;
    });
    if (!previousAnswer) {
      matchingGame.rounds[matchingGame.rounds.length - 1].answers.push(
        req.body
      );
      console.log(util.inspect(matchingGame, { depth: null }));
      res.status(200);
      console.log("Successfully submitted the answer");
      if (
        matchingGame.rounds[matchingGame.rounds.length - 1].answers.length ==
        matchingGame.players.length
      ) {
        matchingGame.phase = "guess";
        console.log("entering guessing phase");
      }
    } else {
      res.status(200);
      previousAnswer.answer = req.body.answer;
      console.log("updated answer");
    }
    res.send();
  }
);

// submit guesses
app.post(
  "/api/v1/game/:id/guess",
  (req: { params: { id: number }; body: Guess }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
      return game.id == req.params.id;
    });
    if (!matchingGame) {
      res.status(404);
      console.log(
        "A game with the specified game id does not exist or the specified player is not part of the game"
      );
      res.send();
      return;
    }
    if (matchingGame.phase != "guess") {
      console.log("not in guessing phase");
      res.status(404);
      res.send();
      return;
    }
    let foundPlayer: String | undefined = matchingGame.players.find(
      (player: String) => {
        return player == req.body.player;
      }
    );
    if (!foundPlayer) {
      res.status(404);
      console.log(
        "A game with the specified game id does not exist or the specified player is not part of the game"
      );
      res.send();
      return;
    }
    let previousGuess: Guess | undefined = matchingGame.rounds[
      matchingGame.rounds.length - 1
    ].guesses.find((guess: Guess) => {
      return guess.player == req.body.player;
    });
    if (!previousGuess) {
      let listOfNamesGuessed: String[] = [];
      req.body.answers.forEach(answer =>
        listOfNamesGuessed.push(answer.player)
      );

      if (listOfNamesGuessed.includes(req.body.player)) {
        console.log("cannot guess yourself");
        res.status(209);
        res.send();
        return;
      }

      let validity: boolean = true;
      matchingGame.players.forEach(player => {
        if (matchingGame != undefined) {
          if (
            (!listOfNamesGuessed.includes(player) &&
              player != req.body.player) ||
            listOfNamesGuessed.length != matchingGame.players.length - 1
          ) {
            console.log("you need to make a guess for every other player");
            res.status(209);
            res.send();
            validity = false;
          }
        }
      });
      if (!validity) {
        return;
      }

      matchingGame.rounds[matchingGame.rounds.length - 1].guesses.push(
        req.body
      );
      if (
        matchingGame.rounds[matchingGame.rounds.length - 1].guesses.length ==
        matchingGame.players.length
      ) {
        matchingGame.rounds.push({
          question: "Answer the question you would have liked to be asked?",
          answers: [],
          guesses: []
        });
        matchingGame.phase = "answer";
        console.log("everyone guessed");
      }
      console.log(util.inspect(matchingGame, { depth: null }));
      res.status(200);
      console.log("Successfully submitted the guesses");
    } else {
      res.status(209);
      console.log("player already submitted a guess");
    }
    res.send();
  }
);

// remove player
app.delete(
  "/api/v1/game/:id",
  (req: { params: { id: number }; body: Player }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
      return game.id == req.params.id;
    });
    if (!matchingGame) {
      res.status(404);
      console.log(
        "A game with the specified game id does not exist or the specified player is not part of the game"
      );
      res.send();
      return;
    }
    let matchingPlayer: String | undefined = matchingGame.players.find(
      (player: String) => {
        return player == req.body.player;
      }
    );
    if (matchingPlayer) {
      removePlayerFromGame(req.body.player, req.params.id);
      matchingGame.rounds[
        matchingGame.rounds.length - 1
      ].answers = matchingGame.rounds[
        matchingGame.rounds.length - 1
      ].answers.filter((answer: Answer) => {
        return answer.player != req.body.player;
      });
      matchingGame.rounds[
        matchingGame.rounds.length - 1
      ].guesses = matchingGame.rounds[
        matchingGame.rounds.length - 1
      ].guesses.filter((guess: Guess) => {
        return guess.player != req.body.player;
      });
      res.status(200);
      console.log("Successfully removed the player from the game");
      console.log(util.inspect(matchingGame, { depth: null }));
    } else {
      res.status(404);
      console.log(
        "A game with the specified game id does not exist or the specified player is not part of the game"
      );
    }
    res.send();
  }
);

app.listen(port, () => console.log(`Example app listening on port ${port}!`));

let games: Game[] = [];

function makeGame(gameId: any): Game {
  let game: Game = {
    id: gameId,
    phase: "answer",
    players: [],
    rounds: [
      {
        question: "Answer the question you would have liked to be asked?",
        answers: [],
        guesses: []
      }
    ]
  };
  games.push(game);
  return game;
}

function addPlayerToGame(player: String, gameId: any): boolean {
  // TODO: make this not break phase of foundGame
  let foundGame = games.find(game => {
    return game.id == gameId;
  });
  if (foundGame) {
    console.log(player);
    foundGame.players.push(player);
    return true;
  }
  return false;
}

function removePlayerFromGame(p: String, gameId: any): boolean {
  let foundGame: undefined | Game = games.find(game => {
    return game.id == gameId;
  });
  if (foundGame) {
    foundGame.players = foundGame.players.filter((player: String) => {
      return player != p;
    });
    return true;
  }
  return false;
}
interface Player {
  player: String;
}

interface Answer {
  player: String;
  answer: String;
}

interface Guess {
  player: String;
  answers: Answer[];
}

interface Round {
  question: String;
  answers: Answer[];
  guesses: Guess[];
}
interface Game {
  phase: "answer" | "guess" | "done";
  id: number;
  players: String[];
  rounds: Round[];
}
