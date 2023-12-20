<script lang="ts">
	import Button from "$lib/Button.svelte";
    import { Game } from "$lib/datatypes/game";
	import { Player } from "$lib/datatypes/player";
	import { onMount } from "svelte";
	import { slide } from "svelte/transition";

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


    async function getGameState() {
        console.log(base_url + game_name);
        const response: Response = await fetch(base_url + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return response;
    }

    async function getGame() {
        getGameState().then((response) => response.json()).then((data) => {
            game = new Game(data.players);
            players = game.players;
            // console.log(data.players);
            // console.log(game.players);
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
        Players:
    </div>
    {#each players as player}
        <div>
            {player.name}
        </div>
    {/each}

</main>