<script>
  import InputField from './lib/InputField.svelte';
  import Button from './lib/Button.svelte'
  let joinGameText = "Join Game"
  let name = "";
  let room_name = "";
  function onClick() {
    console.log(name);
  }

  function onClickJoinRoom() {
    console.log(room_name);
  }
</script>

<main>
  <h1>Weight Inquiries</h1>

  <div class="card">
    <InputField bind:value="{name}" text="enter your name" />
  </div>

  <div class="card">
    <InputField bind:value="{room_name}" text="enter the game room"/>
  </div>
  
  <div class="card">
    <Button text="Join Game" onClick={onClick} />
  </div>

  <div class="card">
    <Button text="Create Game" onClick={onClickJoinRoom}/>
  </div>

</main>
