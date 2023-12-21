<script lang="ts">
	import Button from "$lib/Button.svelte";
	import type { Game } from "$lib/datatypes/game";
	import type { Round } from "$lib/datatypes/round";
	import { Answer } from "$lib/datatypes/answer";
	import { onMount } from "svelte";
	import { Guess } from "$lib/datatypes/guess";
	import Dropdown from "$lib/Dropdown.svelte";
    
    let name: any;
    let game_name: any;
    let base_server_path: any;
    if (typeof localStorage !== "undefined") {
        if (localStorage.getItem("name") != null) {
            name = localStorage.getItem("name");
        }
        else {
            name = ""
        }
    
        if (localStorage.getItem("game_name") != null) {
            game_name = localStorage.getItem("game_name");
        }
        else {
            game_name = ""
        }

        if (localStorage.getItem("base_server_path") != null) {
            base_server_path = localStorage.getItem("base_server_path");
        }
        else {
            base_server_path = ""
        }
    }

    async function getGameState() {
        const response: Response = await fetch(base_server_path + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return response;
    }

    let get_game_interval_ms: number = 1000;
    function sleep(ms: number) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    async function getGameLoop() {
        getGame();
        await sleep(get_game_interval_ms);
        getGameLoop();
    }
    
    onMount(() => {
        getGameLoop();
    })

    function print() {
        console.log("here");
    }

    let game: Game;
    let players: Array<string> = [];
    let answers: Array<Answer> = [];
    let rounds: Array<Round> = [];
    let guess_player_list: Array<string> = [];
    let guess: Guess = new Guess(name, []);
    let has_submitted: boolean = false;
    let has_everybody_guessed: boolean = false;
    async function getGame() {
        getGameState().then((response) => response.json()).then((data) => {
            game = data as Game;
            console.log(game);
            players = data.players;
            if (answers.length == 0) {
                answers = data.rounds[data.rounds.length - 1].answers;
            }
            has_everybody_guessed = data.rounds[data.rounds.length - 1].guesses.length == players.length;
            if (has_everybody_guessed) {
                window.location.href = localStorage.getItem("base_client_path") + "results";
            }
            rounds = game.rounds;
            // has_everybody_answered = game.rounds[game.rounds.length - 1].answers.length == game.players.length;
            // if (has_everybody_answered) {
            //     window.location.href = localStorage.getItem("base_client_path") + "guess";
            // }
            // current_question = game.rounds[rounds.length - 1].question;
        })
    }

    async function postGuess() {
        console.log(JSON.stringify(guess));
        const response: Response = await fetch(base_server_path + game_name + "/guess", {
            method: "POST",
            headers: {"Content-Type": "application/json"},
            body: JSON.stringify(guess),
        })
        return response;
    }

    function onSubmit() {
        console.log(guess_player_list);
        answers.forEach((answer, i) => {
            guess.answers.push(new Answer(guess_player_list[i], answer.answer));    
        });
        console.log(guess);
        const response: Promise<Response> = postGuess();
        response.then((response) => {
            if (response.ok) {
                console.log("ok");
                has_submitted = true;
            }
            else {
                console.log("not ok");
            }
        })
    }

    function onClickPlayer(name: string) {
        guess.answers.push(new Answer(name, answers[0].answer));
        answers.shift();
    }
</script>

<main>
    <div>
        This is the page for guessing who said what
    </div>
    {#if !has_submitted}
        <div>
            {#each answers as answer, i}
                <div>
                    {answer.answer}
                    <Dropdown bind:selected={guess_player_list[i]} options={players} />
                </div>
            {/each}
        </div>
        <div>
            <Button text="Submit" onClick={onSubmit} />
        </div>
    {/if}
    {#if has_submitted}
        wait for the other people
    {/if}
</main>