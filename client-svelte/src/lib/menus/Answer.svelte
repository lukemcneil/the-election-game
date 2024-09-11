<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { Player } from '$lib/datatypes/player';
	import { onMount } from 'svelte';
	import { getGame, postAnswer, postChangeQuestion, postChatGptQuestion } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import PlayerList from '$lib/PlayerList.svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let players: Array<string> = [];
	let players_string: Array<string> = [];
	let current_question: string | undefined = '';
	let round_count: number;
	let waiting_for: Array<string> = []

	let answer: string = '';
	let prompt: string = '';

	function onSubmitClick() {
		if (answer == '') {
			console.log('you need a non-empty answer');
			return;
		}
		const response: Promise<Response> = postAnswer(game_name, name, answer);
		response.then((response) => {
			if (response.ok) {
				setGameState('answer_wait');
			}
		});
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				current_question = data.rounds[data.rounds.length - 1].question;
				round_count = data.rounds.length;
				waiting_for = players.filter(
					(player) =>
						!data.rounds[data.rounds.length - 1].answers.some(
							(answer) => answer.player === player
						)
				);
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'answer') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});

	function onChangeQuestion() {
		const response: Promise<Response> = postChangeQuestion(game_name);
		readGame();
	}

	function onMrGptQuestion() {
		if (prompt == '') {
			return;
		}
		const response: Promise<Response> = postChatGptQuestion(game_name, prompt);
		readGame();
	}
</script>

<main>
	<div class="topleft">
		{game_name}
	</div>
	<div class="topright">
		Round #{round_count}
	</div>
	<PlayerList {players} {waiting_for} />
	<div>
		{current_question}
		{#if name == "ADAM"}
		<Button text="↻" onClick={onChangeQuestion} />
		{/if}
	</div>
	<div>
		<InputField bind:value={answer} text="enter your answer" />
		<Button text="↩" onClick={onSubmitClick} />
	</div>
	<div>
	</div>

		{#if name == "ADAM"}
	<div>Change Question</div>
	<div>
		<InputField bind:value={prompt} text="enter Mr. GPT prompt" />
		<Button text="↩" onClick={onMrGptQuestion} />
	</div>
	{/if}
</main>

<style>
	@import '../../app.css';
</style>
