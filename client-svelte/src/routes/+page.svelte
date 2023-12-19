<script lang="ts">
  import InputField from '../lib/InputField.svelte';
  import Button from '../lib/Button.svelte'
  let production_url = "https://weight-inquiries.onrender.com/api/v1/game/"
  let test_url: string = "http://0.0.0.0:8172/api/v1/game/" 
  let name: string = "";
  let game_name: string = "";

  let pages_base_path: string = "/weighty-inquiry/"

  async function onClickCreateGame() {
    console.log(name);
    const response: Promise<Response> = createGameRequest();
    console.log("here");
    response.then((response) => {
      if (response.ok) {
        console.log("game created " + game_name);
        window.location.href = pages_base_path + "game"
      }
      else {
        console.log("room is already created");
      }
    })
  }

  async function createGameRequest() {
    const request: Promise<Response> = await fetch(production_url + game_name, {
      method: "PUT",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify({
        player: name
      })
    })
    return request;
  }

  async function onClickJoinGame() {
    console.log(game_name);
    const response: Promise<Response> = joinGameRequest();
    response.then((response) => {
      if (response.ok){
        console.log("successfully joined game " + game_name)
        window.location.href = pages_base_path + "game/"
      }
      else {
        console.log("failed to join game " + game_name);
      }
    })
  }

  async function joinGameRequest() {
    const request = await fetch(production_url + game_name, {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify({
        player: name
      })
    })
  }
</script>

<main>

  <nav>
    <a href="/">home</a>
    <a href="/game/">game</a>
    
  </nav>

  <h1>Weight Inquiries</h1>

  <div class="card">
    <InputField bind:value="{name}" text="enter your name" />
  </div>

  <div class="card">
    <InputField bind:value="{game_name}" text="enter the game room"/>
  </div>
  
  <div class="card">
    <Button text="Join Game" onClick={onClickJoinGame} />
  </div>

  <div class="card">
    <Button text="Create Game" onClick={onClickCreateGame}/>
  </div>

</main>
