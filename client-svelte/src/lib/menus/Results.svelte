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
let correct_answer_map: Map < string, string > = new Map();
let my_answer: string;
let my_guess: Array < Answer > = [];
let my_guess_map: Map < string, string > = new Map();
let score_map: Map < string, number > = new Map();
let people_who_guessed_you_correct: Set<string> = new Set([]);

function onNextRoundClick() {
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

            my_guess.forEach((answer: Answer) => {
                my_guess_map.set(answer.player, answer.answer);
            });
        });
}

onMount(() => {
    readGame();
    getScores();
});
</script>

<main>
    <h2>Results</h2>
    <div>
        {question}
    </div>
    <div>
        You said: {my_answer}
    </div>
    <div>
        {#each answers as answer}
        {#if answer.player != name}
            {#if answer.answer == my_guess_map.get(answer.player)}
            <div class="correct">
                <div class="bold">
                {#if people_who_guessed_you_correct.has(answer.player)}⭐️{/if}
                {answer.player}
                {#if people_who_guessed_you_correct.has(answer.player)}⭐️{/if}
                </div>
                {my_guess_map.get(answer.player)}
            </div>
            {:else}
            <div class="incorrect">
                <div class="bold">
                {#if people_who_guessed_you_correct.has(answer.player)}⭐️{/if}
                {answer.player} 
                {#if people_who_guessed_you_correct.has(answer.player)}⭐️{/if}
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
    </div>
    <h2>
        Score
    </h2>
    {#each score_map as [player, score]}
    <div>
        {player}: {score}
    </div>
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
    border:1px solid lightgreen; 
}
.incorrect{
    border:1px solid red; 
}
.bold{
    font-weight: bold;
}
</style>
