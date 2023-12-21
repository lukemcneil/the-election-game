<script lang="ts">
	import Button from "$lib/Button.svelte";

    let name: any;
    let game_name: any;
    let base_server_path: any;
    let round_count: any;
    if (typeof localStorage !== "undefined") {
        if (localStorage.getItem("name") != null) {
            name = localStorage.getItem("name");
        }
        else {
            name = "";
        }
    
        if (localStorage.getItem("game_name") != null) {
            game_name = localStorage.getItem("game_name");
        }
        else {
            game_name = "";
        }

        if (localStorage.getItem("base_server_path") != null) {
            base_server_path = localStorage.getItem("base_server_path");
        }
        else {
            base_server_path = "";
        }

        if (localStorage.getItem("round_count") != null) {
            round_count = localStorage.getItem("round_count");
            round_count = parseInt(round_count);
        }
        else {
            round_count = -1;
        }
    }

    function onNextRoundClick() {
        localStorage.setItem("round_count", round_count + 1 as string);
        window.location.href = localStorage.getItem("base_client_path") + "game";
    }
</script>

<main>
    This is where we will see the results.
    <div>
        <Button text="Next Round" onClick={onNextRoundClick} />
    </div>
</main>