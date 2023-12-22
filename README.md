# Weighty Inquiry

A game of asking questions and guessing answers.

## To Run
1. `cd server-rust`
2. `cargo run -- --questions-file new_questions.txt`

## Design

### User Interactions

- Create a new game with a unique identifier and a unique player identifier
- Join an existing game with a unique player identifier
- Be prompted with the current round's question
- Submit an answer for current round's question
- Guess each player's answer for the current round
- See the correct answers for the current round
- See each players guesses for the current round
- See each players score for the game
- Remove your player from the game

### Server Endpoints

These types and endpoints are optimized for simplicity and ease of development. They are definitely not made for network efficiency or even to prevent cheating.

All messages require the following HTTP headers to be set:

| Headers      | Value            |
| ------------ | ---------------- |
| Content-Type | application/json |

#### Types

##### Player <a name="player"></a>

```js
{
  "player" : "String", // The player with which the request is associated
}
```

##### Answer <a name="answer"></a>

```js
{
  "player" : "String", // The player who gave the answer
  "answer" : "String", // The answer to the question for the round
}
```

##### Guess <a name="guess"></a>

```js
{
  "player" : "String",       // The player making the guess
  "answers" : "Vec<Answer>", // The list of guessed answers, one per player
}
```

##### Round <a name="round"></a>

```js
{
  "question" : "String",      // The question for the round
  "answers"  : "Vec<Answer>", // The list of answers given one per player
  "guesses"  : "Vec<Guess>",  // The list of guesses made, one per player
}
```

##### Game <a name="game"></a>

```js
{
  "players" :  "Vec<String>", // The list of players in the game
  "rounds"  :  "Vec<Round>",  // The list of rounds in the game with the most recent round being the last item in the list
}
```

#### (`PUT`) `/api/v1/game/<game_id>` - Body<[Player](#player)>

Create a new game uniquely identified by `<game_id>` and join the game as the specified player.

**Response**

| Status Code    | Response Body | Description                                      |
| -------------- | ------------- | ------------------------------------------------ |
| 201 (Created)  | empty         | Successfully created the game                    |
| 409 (Conflict) | empty         | A game with the specified game id already exists |

#### (`POST`) `/api/v1/game/<game_id>` - Body<[Player](#player)>

Join an existing game uniquely identified by `<game_id>` with the specified player.

**Response**

| Status Code     | Response Body | Description                                      |
| --------------- | ------------- | ------------------------------------------------ |
| 200 (Ok)        | empty         | Successfully joined the game                     |
| 404 (Not Found) | empty         | A game with the specified game id does not exist |

#### (`GET`) `/api/v1/game/<game_id>`

Get the game state for the game uniquely identified by `<game_id>`.

**Response**

| Status Code     | Response Body | Description                                      |
| --------------- | ------------- | ------------------------------------------------ |
| 200 (Ok)        | [Game](#game) | Successfully retrieved the game's state          |
| 404 (Not Found) | empty         | A game with the specified game id does not exist |

#### (`POST`) `/api/v1/game/<game_id>/answer` - Body<[Answer](#answer)>

Submit an answer for the current round's question an behalf of the specified player.

**Response**

| Status Code     | Response Body | Description                                                                                      |
| --------------- | ------------- | ------------------------------------------------------------------------------------------------ |
| 200 (Ok)        | empty         | Successfully submitted the answer                                                                |
| 404 (Not Found) | empty         | A game with the specified game id does not exist or the specified player is not part of the game |

#### (`POST`) `/api/v1/game/<game_id>/guess` - Body<[Guess](#guess)>

Submit guesses of each players answers for the current round an behalf of the specified player.

**Response**

| Status Code     | Response Body | Description                                                                                      |
| --------------- | ------------- | ------------------------------------------------------------------------------------------------ |
| 200 (Ok)        | empty         | Successfully submitted the guesses                                                               |
| 404 (Not Found) | empty         | A game with the specified game id does not exist or the specified player is not part of the game |

#### (`DELETE`) `/api/v1/game/<game_id>/exit` - Body<[Player](#player)>

Remove a player from the game.

**Response**

| Status Code     | Response Body | Description                                                                                      |
| --------------- | ------------- | ------------------------------------------------------------------------------------------------ |
| 200 (Ok)        | empty         | Successfully removed the player from the game                                                    |
| 404 (Not Found) | empty         | A game with the specified game id does not exist or the specified player is not part of the game |

Delete a game.

#### (`DELETE`) `/api/v1/game/<game_id>`

**Response**

| Status Code     | Response Body | Description                                                                                      |
| --------------- | ------------- | ------------------------------------------------------------------------------------------------ |
| 200 (Ok)        | empty         | Successfully deleted the game                                                                    |
| 404 (Not Found) | empty         | A game with the specified game id does not exist                                                 |

### Client Screens

1. Create or join a game
   - Text input for the name of the game to join
   - Text input for the player name
   - Create button
   - Join button
2. Answer Question
   - The question text for the round
   - Text input for the answer to the questions
   - Submit button
3. Waiting screen while all users submit their answer
4. Guess Answers
   - The question text for the round
   - A column of players
   - A column of guesses
   - Submit button
5. Waiting screen while all users submit their guesses
6. Round results
   - Show the correct answers for the round
   - Show each players guesses
   - Next button
7. Game stats
   - Show each players score
   - Next button

Screens 2 - 7 repeat for the duration of the game
