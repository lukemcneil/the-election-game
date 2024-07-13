<script lang="ts">
    export let text: string;
    export let value = "";
</script>

<input class="input-field shadow" bind:value={value} placeholder={text} />

<style>
	@import '../app.css';
</style>