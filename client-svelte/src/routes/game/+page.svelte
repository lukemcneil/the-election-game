<script lang="ts">
	import Button from "$lib/Button.svelte";

    let name: any;
    let game_name: any;
    if (typeof localStorage !== "undefined") {
        name = localStorage.getItem("name");
        game_name = localStorage.getItem("game_name");
    }

    let get_game_request: string = "api/v1/game/";

    async function getGameState() {
        const request: Promise<Request> = await fetch(get_game_request + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return request;
    }

    function print() {
        console.log(name);
        console.log(game_name);
    }
</script>

<main>
    <Button text="test" onClick={print} />
    <div>
        {name}
    </div>
    <div>
        {game_name}
    </div>
    this is the game page
</main>