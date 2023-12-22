<script lang="ts">
  import InputField from '../lib/InputField.svelte';
  import Button from '../lib/Button.svelte'
	import { onMount } from 'svelte';
  
  let production_url = "https://weight-inquiries.onrender.com/api/v1/game/"
  let test_url: string = "http://0.0.0.0:8172/api/v1/game/"
  
  let name: string = "";
  let game_name: string = "";

  let error_message = "";
  let no_name_error_message = "no name";
  let no_game_room_error_message = "no game room name";
  let game_already_exists_error_message = "this game already exists";

  function updateGlobal() {
    localStorage.setItem("name", name);
    localStorage.setItem("game_name", game_name);
  }

  async function onClickCreateGame() {
    if (name == "") {
      error_message = no_name_error_message;
      return;
    }
    if (game_name == "") {
      error_message = no_game_room_error_message;
      return;
    }
    const response: Promise<Response> = createGameRequest();
    response.then((response) => {
      if (response.ok) {
        updateGlobal();
        window.location.href = window.location.href + "game"
      }
      else {
        console.log(response.status);
        if (response.status == 409) {
          error_message = game_already_exists_error_message;
        }
        error_message = "some other error when making a game";
      }
    })
  }

  async function createGameRequest() {
    const response: Response = await fetch(localStorage.getItem("base_server_path") + game_name, {
      method: "PUT",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify({
        player: name
      })
    })
    return response;
  }

  async function onClickJoinGame() {
    if (name == "") {
      error_message = no_name_error_message;
      return;
    }
    if (game_name == "") {
      error_message = no_game_room_error_message;
      return;
    }
    const response: Promise<Response> = joinGameRequest();
    response.then((response) => {
      if (response.ok){
        updateGlobal();
        localStorage.setItem("round_count", "0");
        window.location.href = window.location.href + "game"
      }
      else {
        console.log("failed to join game " + game_name);
      }
    })
  }

  async function joinGameRequest() {
    const request = await fetch(localStorage.getItem("base_server_path") + game_name, {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify({
        player: name
      })
    })
    return request;
  }

  onMount(() => {
    localStorage.setItem("base_client_path", window.location.href);
    console.log(window.location.href);
    if (window.location.href == "http://localhost:5173/") {
      localStorage.setItem("base_server_path", test_url);
    }
    else {
      localStorage.setItem("base_server_path", production_url);
    }
    localStorage.setItem("has_answered", "false");
    localStorage.setItem("has_guessed", "false");
    localStorage.setItem("round_count", "");
  })
</script>

<style>
  @import '../app.css';
</style>

<main>
  <h1>Weight Inquiries</h1>

  <div>
    <InputField bind:value="{name}" text="enter your name" />
  </div>

  <div>
    <InputField bind:value="{game_name}" text="enter the game room"/>
  </div>
  
  <div>
    <Button text="Join Game" onClick={onClickJoinGame} />
  </div>

  <div>
    <Button text="Create Game" onClick={onClickCreateGame}/>
  </div>

  <div>
    {error_message}
  </div>

</main>
