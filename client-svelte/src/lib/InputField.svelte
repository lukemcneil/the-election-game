<script lang="ts">
    export let text: string;
    export let value = "";

  function handleInput(event) {
    let inputValue = event.target.value.toUpperCase();
    value = inputValue;
  }
</script>

<input class="input-field shadow" bind:value placeholder={text} on:input={handleInput} />

<style>
	@import '../app.css';
</style>