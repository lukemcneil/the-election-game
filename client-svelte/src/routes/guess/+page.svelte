<script lang="ts">
    
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
        console.log(base_server_path);
        const response: Response = await fetch(base_server_path + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return response;
    }
</script>

<main>
    <div>
        This is the page for guessing who said what
    </div>
</main>