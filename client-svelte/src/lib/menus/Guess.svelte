<script lang="ts">
	import Button from '$lib/Button.svelte';
	import type { Round } from '$lib/datatypes/round';
	import { Answer } from '$lib/datatypes/answer';
	import { onMount } from 'svelte';
	import { Guess } from '$lib/datatypes/guess';
	import { getGame, postGuess } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import Dropdown from '$lib/Dropdown.svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let question: string;
	let players: Array<string> = [];
	// let other_players: Array<string> = [];
	let answers: Array<Answer> = [];
	let rounds: Array<Round> = [];
	let guess_player_list: Array<string> = [];
	let guess: Guess = new Guess(name, []);

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
                question = data.rounds[data.rounds.length - 1].question;
				players = data.players;
                answers = data.rounds[data.rounds.length - 1].answers;
				// players.forEach((player: string) => {
				// 	if (player != name) {
				// 		other_players.push(player);
				// 	}
				// });
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'guess') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});

	function onSubmit() {
		answers.forEach((answer, i) => {
			if (answer.player != name) {
				guess.answers.push(new Answer(guess_player_list[i], answer.answer));
			}
		});
		const response: Promise<Response> = postGuess(game_name, guess);
		response.then((response) => {
			if (response.ok) {
				setGameState('guess_wait');
			}
		});
	}
</script>

<main>
	<h2>Guess who said what</h2>
	<div>{question}</div>
	<div>
		{#each answers as answer, i}
			{#if answer.player != name}
				<div>
					{answer.answer}
					<Dropdown bind:selected={guess_player_list[i]} options={players} />
				</div>
			{/if}
		{/each}
	</div>
	<div>
		<Button text="Submit" onClick={onSubmit} />
	</div>
</main>

<style>
	@import '../../app.css';
</style>
