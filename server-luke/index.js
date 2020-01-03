const express = require('express')
const app = express()
const port = 3000
app.use(express.json());
const util = require('util')

// TODO: store current round somewhere
let currentRound = 0;

// create game
app.put('/api/v1/game/:id', (req, res) => {
    let check = false;
    games.map((game) => {
        if (game.id == req.params.id) {
            res.status(409);
            console.log('A game with the specified game id already exists');
            check = true;
        }
    })
    if (!check) {
        let madeGame = makeGame(req.params.id);
        addPlayerToGame(req.body.player, madeGame.id)
        console.log(util.inspect(madeGame, { depth: null }))
        res.status(201);
        console.log('Successfully created the game');
    }
    res.send();
})

// add player
app.post('/api/v1/game/:id', (req, res) => {
    let matchingGame = games.find((game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log('A game with the specified game id does not exist');
        return;
    }
    let matchingPlayer = matchingGame.players.find((player) => {
        return player == req.body.player;
    })
    if (matchingPlayer == undefined) {
        addPlayerToGame(req.body.player, req.params.id)
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
app.get('/api/v1/game/:id', (req, res) => {
    let matchingGame = games.find((game) => {
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
app.post('/api/v1/game/:id/answer', (req, res) => {
    let matchingGame = games.find((game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log("A game with the specified game id does not exist or the specified player is not part of the game");
        res.send();
        return;
    }
    let foundPlayer = matchingGame.players.find((player) => {
        return player == req.body.player;
    })
    if (!foundPlayer) {
        res.status(404);
        console.log("A game with the specified game id does not exist or the specified player is not part of the game");
        res.send();
        return;
    }
    let previousAnswer = matchingGame.rounds[currentRound].answers.find((answer) => {
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
app.post('/api/v1/game/:id/guess', (req, res) => {
    let matchingGame = games.find((game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log("A game with the specified game id does not exist or the specified player is not part of the game");
        res.send();
        return;
    }
    let foundPlayer = matchingGame.players.find((player) => {
        return player == req.body.player;
    })
    if (!foundPlayer) {
        res.status(404);
        console.log("A game with the specified game id does not exist or the specified player is not part of the game");
        res.send();
        return;
    }
    let previousGuess = matchingGame.rounds[currentRound].guesses.find((guess) => {
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
app.delete('/api/v1/game/:id', (req, res) => {
    let matchingGame = games.find((game) => {
        return game.id == req.params.id;
    })
    if (!matchingGame) {
        res.status(404);
        console.log('A game with the specified game id does not exist or the specified player is not part of the game');
        res.send();
        return;
    }
    let matchingPlayer = matchingGame.players.find((player) => {
        return player == req.body.player;
    })
    if (matchingPlayer) {
        removePlayerFromGame(req.body.player, req.params.id);
        matchingGame.rounds[currentRound].answers = matchingGame.rounds[currentRound].answers.filter((answer) => {
            return answer.player != req.body.player;
        })
        matchingGame.rounds[currentRound].guesses = matchingGame.rounds[currentRound].guesses.filter((guess) => {
            return guess.player != req.body.player;
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

let games = [];

function makeGame(gameId) {
    let game = {
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

function addPlayerToGame(player, gameId) {
    let foundGame = games.find((game) => {
        return game.id == gameId;
    });
    foundGame.players.push(player);
}

function removePlayerFromGame(p, gameId) {
    let foundGame = games.find((game) => {
        return game.id == gameId;
    })
    foundGame.players = foundGame.players.filter((player) => {
        return player != p;
    })
}