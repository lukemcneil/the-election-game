<script lang="ts">
    export let players: Array<string>
    export let waiting_for: Array<string>

	function answering(player: string) {
		for (let i = 0; i < waiting_for.length; i++) {
			if (waiting_for[i] == player) {
				return true;
			}
		}
		return false;
	}
</script>

	<h2>Players</h2>
	<div class="flex-container">
		{#each players as player}
			{#if (answering(player))}
				<div class="waiting item shadow">
					{player}
				</div>
			{:else}
				<div class="item shadow">
					{player}
				</div>
			{/if}
		{/each}
	</div>

<style>
	@import '../app.css';
</style>