<script lang="ts">
	import type { svelte } from "@sveltejs/vite-plugin-svelte";

    let game_name: string = "";
    let get_game_request: string = "api/v1/game/";

    import { onMount } from svelte;

    function onMount() {
        
    }

    async function getGameState() {
        const request: Promise<Request> = await fetch(get_game_request + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return request;
    }
</script>

<main>

</main>