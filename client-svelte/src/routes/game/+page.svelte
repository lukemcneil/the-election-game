<script lang="ts">
	import Button from "$lib/Button.svelte";
	import InputField from "$lib/InputField.svelte";
    import { Game } from "$lib/datatypes/game";
	import { Player } from "$lib/datatypes/player";
    import { Round } from "$lib/datatypes/round";
	import { onMount } from "svelte";

    let name: string | null;
    let game_name: string | null;
    if (typeof localStorage !== "undefined") {
        name = localStorage.getItem("name");
        game_name = localStorage.getItem("game_name");
    }

    let production_url: string = "https://weight-inquiries.onrender.com/api/v1/game/"
    let test_url: string = "http://0.0.0.0:8172/api/v1/game/"
    let base_url: string = test_url;

    let game: Game;
    let players: Array<Player> = [];
    let rounds: Array<Round> = [];
    
    let current_question: string | undefined = "";
    let answer: string = "";

    async function getGameState() {
        const response: Response = await fetch(base_url + game_name, {
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
                window.location.href = window.location.href + "wait"
            }
        })
    }

    async function postAnswer() {
        const response: Response = await fetch(base_url + game_name + "/answer", {
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
            current_question = game.rounds[rounds.length - 1].question;
            console.log(rounds);
        })
    }
    
    let get_game_interval_ms: number = 1000;
    function sleep(ms: number) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    async function getGameLoop() {
        getGame();
        await sleep(get_game_interval_ms);
        // getGameLoop();
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
    <div>
        <InputField bind:value="{answer}" text="enter your answer" />
    </div>
    <div>
        <Button text="Submit" onClick={onSubmitClick} />
    </div>
    <!-- <div>
        {rounds[rounds.length - 1].question}
    </div> -->
    <div>
        Players:
    </div>
    {#each players as player}
        <div>
            {player}
        </div>
    {/each}

</main>