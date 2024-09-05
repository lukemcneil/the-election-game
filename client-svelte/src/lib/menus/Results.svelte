<script lang="ts">
	import Button from '$lib/Button.svelte';
	import type { Answer } from '$lib/datatypes/answer';
	import type { Picture } from '$lib/datatypes/picture';
	import type { Guess } from '$lib/datatypes/guess';
	import { Score } from '$lib/datatypes/score';
	import { onMount } from 'svelte';
	import { getGame, getScore } from '$lib/functions/requests';
	import type { Game } from '$lib/datatypes/game';
	import type { Round } from '$lib/datatypes/round';
	import GuessWait from './GuessWait.svelte';
	import Guess from './Guess.svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let question: string;
	let answers: Array<Answer> = [];

	let pictures: Array<Picture> = [];
	let players: Array<string> = [];
	let correct_answer_map: Map<string, string> = new Map();
	let my_answer: string;
	let my_guess: Array<Answer> = [];
	let my_guess_map: Map<string, string> = new Map();
	let score_map: Map<string, number> = new Map();
	let knows_score_map: Map<string, number> = new Map();
	let people_who_guessed_you_correct: Set<string> = new Set([]);
	let round_count: number;
	let game: Game;

	function onNextRoundClick() {
		setGameState('answer');
	}

	function getScores() {
		getScore(game_name)
			.then((response) => response.json())
			.then((data) => {
				for (var prop in data) {
					score_map.set(prop, data[prop]);
				}
				score_map = new Map([...score_map.entries()].sort((a, b) => b[1] - a[1]));
			});
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				game = data;
				round_count = data.rounds.length;
				players = data.players;
				question = data.rounds[data.rounds.length - 2].question;
				answers = data.rounds[data.rounds.length - 2].answers;
				pictures = data.rounds[data.rounds.length - 2].pictures;

				answers.forEach((answer: Answer) => {
					correct_answer_map.set(answer.player, answer.answer);
				});
				my_answer = correct_answer_map.get(name);

				data.rounds[data.rounds.length - 2].guesses.forEach((guess: Guess) => {
					if (guess.player == name) {
						my_guess = guess.answers;
					} else {
						guess.answers.forEach((answer) => {
							if (answer.player == name && answer.answer == my_answer) {
								people_who_guessed_you_correct.add(guess.player);
							}
						});
					}
				});
				people_who_guessed_you_correct = people_who_guessed_you_correct;

				my_guess.forEach((answer: Answer) => {
					my_guess_map.set(answer.player, answer.answer);
				});

				data.players.forEach((player: string) => {
					if (player != name) {
						console.log(player);
						getKnowScore(player);
					}
				});
				knows_score_map = new Map([...knows_score_map.entries()].sort((a, b) => b[1] - a[1]));
			});
	}

	async function handleOnMount() {}

	onMount(() => {
		readGame();
		getScores();
	});

	function didYouGuessRight(player: string) {
		return my_guess_map.get(player) == correct_answer_map.get(player);
	}

	function showCorrectAnswer(player: string) {
		alert('For ' + player + ' you guessed: ' + my_guess_map.get(player));
	}

	function getLeaderBoardCssClass(player: string) {
		if (player == name) {
			return 'me';
		} else if (didYouGuessRight(player)) {
			return 'correct';
		} else {
			return 'incorrect';
		}
	}

	function getWhoKnowsCssClass(player: string) {
		if (people_who_guessed_you_correct.has(player)) {
			return 'correct';
		} else {
			return 'incorrect';
		}
	}

	function getKnowScore(player: string) {
		console.log(player);
		console.log(game);
		let count: number = 0;
		game?.rounds.forEach((round: Round) => {
			let correct_answer = '';
			round.answers.forEach((answer: Answer) => {
				if (answer.player == name) {
					correct_answer = answer.answer;
					return;
				}
			});

			round.guesses.forEach((guess: Guess) => {
				if (guess.player == player) {
					return guess.answers.forEach((answer: Answer) => {
						if (answer.answer == correct_answer && answer.player == name) {
							count += 1;
						}
					});
				}
			});
			console.log(round);
		});
		knows_score_map.set(player, count);
	}
</script>

<main>
	<div class="topleft">
		{game_name}
	</div>
	<div class="topright">
		Round #{round_count}
	</div>

	<div>
		{question}
	</div>

	<h2>Leader Board</h2>
	{#each score_map as [player, score]}
		<div
			class="{getLeaderBoardCssClass(player)}"
			on:click={() => showCorrectAnswer(player)}
		>
			{player}: {score}
			<div>
				"{correct_answer_map.get(player)}"
			</div>
		</div>
	{/each}

	<h2>Who guessed you correct</h2>
	{#each knows_score_map as [player, score]}
		<div class="{getWhoKnowsCssClass(player)}">
			{player}: {score}
		</div>
	{/each}

	<div>
		<Button text="Next Round" onClick={onNextRoundClick} />
	</div>
</main>

<style>
	@import '../../app.css';
	.correct {
		background-color: rgba(116, 185, 95, 0.75);
	}
	.incorrect {
		background-color: rgba(202, 96, 96, 0.75);
	}
	.me {
		background-color: rgba(28, 188, 252, 0.75);
	}
</style>
