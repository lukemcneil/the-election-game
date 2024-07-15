<script lang="ts">
import Button from '$lib/Button.svelte';
import type {
    Answer
} from '$lib/datatypes/answer';
import type {
    Picture
} from '$lib/datatypes/picture';
import type {
    Guess
} from '$lib/datatypes/guess';
import {
    Score
} from '$lib/datatypes/score';
import {
    onMount
} from 'svelte';
import {
    getGame,
    getScore
} from '$lib/functions/requests';

export let setGameState: (new_state: string) => void;
export let name: string | null;
export let game_name: string | null;

let question: string;
let answers: Array < Answer > = [];

let pictures: Array < Picture > = [];
let players: Array<string> = [];
let correct_answer_map: Map < string, string > = new Map();
let my_answer: string;
let my_guess: Array < Answer > = [];
let my_guess_map: Map < string, string > = new Map();
let score_map: Map < string, number > = new Map();
let people_who_guessed_you_correct: Set<string> = new Set([]);
let round_count: number;

function onNextRoundClick() {
    localStorage.setItem("get_increment", "true");
    setGameState('answer');
}

function getScores() {
    getScore(game_name)
        .then((response) => response.json())
        .then((data) => {
            for (var prop in data) {
                score_map.set(prop, data[prop]);
            }
            score_map = new Map([...score_map.entries()].sort((a, b) => b[1] - a[1]));
        });
}

async function readGame() {
    getGame(game_name)
        .then((response) => response.json())
        .then((data) => {
            round_count = data.rounds.length
            players = data.players;
            question = data.rounds[data.rounds.length - 2].question;
            answers = data.rounds[data.rounds.length - 2].answers;
            pictures = data.rounds[data.rounds.length - 2].pictures;

            answers.forEach((answer: Answer) => {
                correct_answer_map.set(answer.player, answer.answer);
            });
            my_answer = correct_answer_map.get(name);

            data.rounds[data.rounds.length - 2].guesses.forEach((guess: Guess) => {
                if (guess.player == name) {
                    my_guess = guess.answers;
                } else {
                    guess.answers.forEach(answer => {
                        if (answer.player == name && answer.answer == my_answer) {
                            people_who_guessed_you_correct.add(guess.player);
                        }
                    });
                }
                
            });
            people_who_guessed_you_correct = people_who_guessed_you_correct;
            if (localStorage.getItem("get_increment") == "true") {
                people_who_guessed_you_correct.forEach(player => {
                    if (localStorage.getItem(player)) {
                        localStorage.setItem(player, localStorage.getItem(player) + "x")
                    } else {
                        localStorage.setItem(player, "x")
                    }
                    
                });
                localStorage.setItem("get_increment", "false");
            }

            my_guess.forEach((answer: Answer) => {
                my_guess_map.set(answer.player, answer.answer);
            });
        });
}

onMount(() => {
    readGame();
    getScores();
});

function didYouGuessRight(player: string) {
    return my_guess_map.get(player) == correct_answer_map.get(player);
}

function on_click(player: string) {
    alert("For " + player + " you guessed: " + my_guess_map.get(player));
}
</script>

<main>
	<div class="topleft">
		{game_name}
	</div>
	<div class="topright">
		Round #{round_count}
	</div>
    <!-- <div>
        {#each answers as answer}
        {#if answer.player != name}
            {#if answer.answer == my_guess_map.get(answer.player)}
            <div class="correct">
                <div class="bold">
                {answer.player}
                </div>
                {my_guess_map.get(answer.player)}
            </div>
            {:else}
            <div class="incorrect">
                <div class="bold">
                {answer.player} 
                </div>
                    {answer.answer}
                <div>
                    You guessed:
                </div>
                {my_guess_map.get(answer.player)}
            </div>
            {/if}
        {/if}
        {/each}
    </div> -->
    <div>
        {question}
    </div>
    <h2>
        Leader Board
    </h2>
    {#each score_map as [player, score]}
        {#if player == name}
            <div class="leader-board me">
                {player}: {score}
                <div>
                    "{correct_answer_map.get(player)}"
                </div>
            </div>
        {:else if didYouGuessRight(player)}
            <div class="leader-board correct">
                {player}: {score}
                <div>
                    "{correct_answer_map.get(player)}"
                </div>
            </div>
        {:else}
            <div class="leader-board incorrect" on:click={() => on_click(player)}>
                {player}: {score}
                <div>
                    "{correct_answer_map.get(player)}"
                </div>
            </div>
        {/if}
    {/each}
    <h2>
        Who guessed you correct
    </h2>
    {#each players as player}
    {#if localStorage.getItem(player)}
            {#if people_who_guessed_you_correct.has(player)}
        <div class="leader-board correct">
            {player}: {localStorage.getItem(player)?.length}
        </div>
            {:else}
        <div class="leader-board incorrect">
            {player}: {localStorage.getItem(player)?.length}
        </div>
            {/if}

    {:else if player != name}
        <div class="leader-board">
            {player}: 0
        </div>
    {/if}
    {/each}
    <div>
        <Button text="Next Round" onClick={onNextRoundClick} />
    </div>
    <!-- <div>
        {#each pictures as picture}
        <div>{picture.prompt}</div>
        <div>
            <img src="{localStorage.getItem("base_server_path")?.replace("api/v1/game/", "")}{picture.url}" alt="Shut up"/>

        </div>
        {/each}
    </div> -->
</main>

<style>
@import '../../app.css';
.correct{
    /* border:5px solid rgb(177, 253, 177);  */
    background-color: #74b95f; 
}
.incorrect{
    /* border:5px solid rgb(255, 128, 128);  */
    background-color: #ca6060; 
}
.me{
    background-color: #1cbcfc; 
}
.bold{
    font-weight: bold;
}
.leader-board {
  padding: 10px; /* Padding around content */
  margin: 5px; /* Margin between items */
  font-family: inherit; /* Font family */
  font-size: 16px; /* Font size */
  text-align: center; /* Center text */
  flex: 1; /* Equal width distribution among items */
  border-radius: 5px;
  flex: 0 0 calc(25% - 10px);

}
</style>
