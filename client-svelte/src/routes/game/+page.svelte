<script lang="ts">
	import Button from "$lib/Button.svelte";
    import { Game } from "$lib/datatypes/game";
	import { Player } from "$lib/datatypes/player";

    let name: any;
    let game_name: any;
    if (typeof localStorage !== "undefined") {
        name = localStorage.getItem("name");
        game_name = localStorage.getItem("game_name");
    }

    let production_url = "https://weight-inquiries.onrender.com/api/v1/game/"
    let test_url: string = "http://0.0.0.0:8172/api/v1/game/"
    let base_url = test_url;

    let game: Game;
    let players: Array<Player> = [];


    async function getGameState() {
        console.log(base_url + game_name);
        const response: Promise<Response> = await fetch(base_url + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return response;
    }

    async function getGame() {
        getGameState().then((response) => response.json()).then((data) => {
            game = new Game(data.players);
            players = game.players;
            console.log(data.players);
            console.log(game.players);
        })
    }

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