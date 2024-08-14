<script lang="ts">
    export let text: string;
    export let onClick = () => (console.log("hello world"));
</script>

{#if text.length == 1}	
<button class="small-button shadow" on:click={onClick}> 
    {text}
</button>
{:else}
<button class="button shadow" on:click={onClick}> 
    {text}
</button>
{/if}

<style>
	@import '../app.css';
</style>
