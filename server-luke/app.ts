const express = require('express')
const app = express()
const port = 3000
app.use(express.json());
const util = require('util')

// TODO: store current round somewhere
let currentRound = 0;

// create game
app.put('/api/v1/game/:id', (req: { params: { id: number; }; body: Player; }, res: any) => {
    let check: boolean = false;
    games.map((game: Game) => {
        if (game.id == req.params.id) {
            res.status(409);
            console.log('A game with the specified game id already exists');
            check = true;
        }
    })
    if (!check) {
        let madeGame = makeGame(req.params.id);
        console.log(req.body);
        addPlayerToGame(req.body, madeGame.id)
        console.log(util.inspect(madeGame, { depth: null }))
        res.status(201);
        console.log('Successfully created the game');
    }
    res.send();
})

// add player
app.post('/api/v1/game/:id', (req: { params: { id: number; }; body: Player; }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log('A game with the specified game id does not exist');
        res.send();
        return;
    }
    let matchingPlayer: Player | undefined = matchingGame.players.find((player: Player) => {
        return player == req.body;
    })
    if (!matchingPlayer) {
        addPlayerToGame(req.body, req.params.id)
        res.status(200);
        console.log('Successfully joined the game');
        console.log(util.inspect(matchingGame, { depth: null }))
    } else {
        res.status(409)
        console.log('A player with the specified game id already exists');
    }
    res.send();
})

// get game data
app.get('/api/v1/game/:id', (req: { params: { id: number; }; }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log('A game with the specified game id does not exist');
        res.send();
        return;
    }
    console.log(util.inspect(matchingGame, { depth: null }))
    res.status(200);
    res.json(matchingGame);
})

//  submit answer
app.post('/api/v1/game/:id/answer', (req: { params: { id: number; }; body: Answer; }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log("A game with the specified game id does not exist or the specified player is not part of the game");
        res.send();
        return;
    }
    let foundPlayer: Player | undefined = matchingGame.players.find((player: Player) => {
        return player.name == req.body.player.name;
    })
    if (!foundPlayer) {
        res.status(404);
        console.log("A game with the specified game id does not exist or the specified player is not part of the game");
        res.send();
        return;
    }
    let previousAnswer: Answer | undefined = matchingGame.rounds[currentRound].answers.find((answer: Answer) => {
        return answer.player == req.body.player;
    })
    if (!previousAnswer) {
        matchingGame.rounds[currentRound].answers.push(req.body);
        console.log(util.inspect(matchingGame, { depth: null }))
        res.status(200);
        console.log("Successfully submitted the answer")
    } else {
        res.status(209);
        console.log("player already submitted an answer");
    }
    res.send();
})

// submit guesses
app.post('/api/v1/game/:id/guess', (req: { params: { id: number; }; body: Guess; }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log("A game with the specified game id does not exist or the specified player is not part of the game");
        res.send();
        return;
    }
    let foundPlayer: Player | undefined = matchingGame.players.find((player: Player) => {
        return player.name == req.body.player.name;
    })
    if (!foundPlayer) {
        res.status(404);
        console.log("A game with the specified game id does not exist or the specified player is not part of the game");
        res.send();
        return;
    }
    let previousGuess: Guess | undefined = matchingGame.rounds[currentRound].guesses.find((guess: Guess) => {
        return guess.player == req.body.player;
    })
    if (!previousGuess) {
        matchingGame.rounds[currentRound].guesses.push(req.body);
        console.log(util.inspect(matchingGame, { depth: null }))
        res.status(200);
        console.log("Successfully submitted the guesses")
    } else {
        res.status(209);
        console.log("player already submitted a guess");
    }
    res.send();
})

// remove player
app.delete('/api/v1/game/:id', (req: { params: { id: number; }; body: Player; }, res: any) => {
    let matchingGame: Game | undefined = games.find((game: Game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log('A game with the specified game id does not exist or the specified player is not part of the game');
        res.send();
        return;
    }
    let matchingPlayer: Player | undefined = matchingGame.players.find((player: Player) => {
        return player.name == req.body.name;
    })
    if (matchingPlayer) {
        removePlayerFromGame(req.body.name, req.params.id);
        matchingGame.rounds[currentRound].answers = matchingGame.rounds[currentRound].answers.filter((answer: Answer) => {
            return answer.player.name != req.body.name;
        })
        matchingGame.rounds[currentRound].guesses = matchingGame.rounds[currentRound].guesses.filter((guess: Guess) => {
            return guess.player.name != req.body.name;
        })
        res.status(200);
        console.log('Successfully removed the player from the game');
        console.log(util.inspect(matchingGame, { depth: null }))
    } else {
        res.status(404)
        console.log('A game with the specified game id does not exist or the specified player is not part of the game');
    }
    res.send();
})

app.listen(port, () => console.log(`Example app listening on port ${port}!`))

let games: Game[] = [];

function makeGame(gameId: any): Game {
    let game: Game = {
        id: gameId,
        players: [],
        rounds: [{
            question: "example question",
            answers: [],
            guesses: []
        }]
    }
    games.push(game)
    return game;
}

function addPlayerToGame(player: Player, gameId: any): boolean {
    let foundGame = games.find((game) => {
        return game.id == gameId;
    });
    if (foundGame) {
        console.log(player)
        foundGame.players.push(player);
        return true;
    }
    return false;
}

function removePlayerFromGame(p: String, gameId: any): boolean {
    let foundGame: undefined | Game = games.find((game) => {
        return game.id == gameId;
    })
    if (foundGame) {
        foundGame.players = foundGame.players.filter((player: Player) => {
            return player.name != p;
        })
        return true;
    }
    return false;
}
interface Player {
    name: String;
}

interface Answer {
    player: Player;
    answer: String;
}

interface Guess {
    player: Player;
    answers: Answer[];
}

interface Round {
    question: String;
    answers: Answer[];
    guesses: Guess[];
}
interface Game {
    id: number;
    players: Player[];
    rounds: Round[];
}