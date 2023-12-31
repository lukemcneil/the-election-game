<script lang="ts">
	import Button from '$lib/Button.svelte';
	import Join from '$lib/menus/Join.svelte';
	import Answer from '$lib/menus/Answer.svelte';
	import { onMount } from 'svelte';
	import AnswerWait from '$lib/menus/AnswerWait.svelte';
	import Guess from '$lib/menus/Guess.svelte';
	import GuessWait from '$lib/menus/GuessWait.svelte';
	import Results from '$lib/menus/Results.svelte';

	let game_state: string | null;

	let production_url: string = 'https://weight-inquiries.onrender.com/api/v1/game/';
	let test_url: string = 'http://0.0.0.0:8172/api/v1/game/';

	function setGameState(new_state: string) {
		localStorage.setItem('game_state', new_state);
		game_state = new_state;
	}

	onMount(() => {
		if (!localStorage.getItem('game_state')) {
			setGameState('join');
		} else {
			game_state = localStorage.getItem('game_state');
		}
		if (window.location.href == 'http://localhost:5173/') {
			localStorage.setItem('base_server_path', test_url);
		} else {
			localStorage.setItem('base_server_path', production_url);
		}
	});

	function reset() {
		if (confirm('Do you really want to leave the game?') == true) {
			setGameState('join');
		}
	}
</script>

<main>
	<!-- {#if score_header_states.has(game_state)}
		<ScoreHeader
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
		/>
	{/if} -->

	{#if game_state == 'join'}
		<Join {setGameState} />
	{:else if game_state == 'answer'}
		<Answer
			{setGameState}
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
		/>
	{:else if game_state == 'answer_wait'}
		<AnswerWait {setGameState} game_name={localStorage.getItem('game_name')} />
	{:else if game_state == 'guess'}
		<Guess
			{setGameState}
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
		/>
	{:else if game_state == 'guess_wait'}
		<GuessWait {setGameState} game_name={localStorage.getItem('game_name')} />
	{:else if game_state == 'results'}
		<Results
			{setGameState}
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
		/>
	{/if}
	<div style="padding-top: 10em;">
		<Button text="Leave Game" onClick={reset} />
	</div>
</main>

<style>
	@import '../app.css';
</style>
