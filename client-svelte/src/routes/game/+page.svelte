<script lang="ts">
	import Button from "$lib/Button.svelte";
	import InputField from "$lib/InputField.svelte";
    import { Game } from "$lib/datatypes/game";
	import { Player } from "$lib/datatypes/player";
    import { Round } from "$lib/datatypes/round";
	import { onMount } from "svelte";

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

    let game: Game;
    let players: Array<Player> = [];
    let rounds: Array<Round> = [];
    
    let current_question: string | undefined = "";
    let answer: string = "";
    let has_answered: boolean = false;
    let has_everybody_answered: boolean = false;

    async function getGameState() {
        const response: Response = await fetch(base_server_path + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return response;
    }

    function onSubmitClick() {
        if (answer == "" ) {
            console.log("you need a non-empty answer");
            return;
        }
        const response: Promise<Response> = postAnswer(); 
        response.then((response) => {
            if (response.ok) {
                // window.location.href = localStorage.getItem("base_client_path") + "wait"
                has_answered = true;
            }
        })
    }

    async function postAnswer() {
        const response: Response = await fetch(base_server_path + game_name + "/answer", {
            method: "POST",
            headers: {"Content-Type": "application/json"},
            body: JSON.stringify({
                player: name,
                answer: answer,
            })
        })
        return response;
    }

    async function getGame() {
        getGameState().then((response) => response.json()).then((data) => {
            game = data as Game;
            players = game.players;
            rounds = game.rounds;
            has_everybody_answered = game.rounds[game.rounds.length - 1].answers.length == game.players.length;
            if (has_everybody_answered) {
                window.location.href = localStorage.getItem("base_client_path") + "guess";
            }
            current_question = game.rounds[rounds.length - 1].question;
        })
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
        console.log(game.players);
    }
</script>

<main>
    <Button text="getGame" onClick={getGame} />
    <Button text="print" onClick={print}/>
    <div>
        Game Room Name: {game_name}
    </div>
    <div>
        {current_question}
    </div>
    {#if !has_answered}
        <div>
            <InputField bind:value="{answer}" text="enter your answer" />
        </div>
        <div>
            <Button text="Submit" onClick={onSubmitClick} />
        </div>
    {/if}
    <div>
        Players:
    </div>
    {#each players as player}
        <div>
            {player}
        </div>
    {/each}

    {#if has_answered}
    <div>
        you have answered the question
    </div>
    {/if}

</main>