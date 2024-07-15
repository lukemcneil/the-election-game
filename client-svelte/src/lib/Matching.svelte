<script src="DragDropTouch.js" lang="ts">
	import { flip } from 'svelte/animate';
	import { Picture } from '$lib/datatypes/picture';
	import { onMount } from 'svelte';
	import { sleep } from './functions/helper';


	export let baskets: Array<{ name: string; item: string }> = [];
	export let players: Array<string> = [];
	export let pictures: Array<Picture> = []

	let selected_name: string | null;
	let selected_basket_id: number | null;

	function geturl(prompt : string){
		for (let i = 0; i < pictures.length; i++){
			console.log(pictures[i].prompt + " " + prompt);
			if (prompt == pictures[i].prompt){
				return pictures[i].url;
			}
		} 
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		console.log(selected_name);
		console.log(selected_basket_id);
		await sleep(get_game_interval_ms);
		getGameLoop();
	}

	onMount(() => {
		// getGameLoop();
	});

	function isSameBasket(basket_id: number) {
		if (selected_name == null) {
			return false;
		} else if (basket_id == -1) {
			return selected_name in players;
		} else {
			if (baskets[basket_id].item == selected_name) {
				return true;
			}
		}
	}

	function onClickBasket(event: any, basket_id: number) {
		console.log("Basket")
		if (selected_name == null) {
			return;
		} else if (isSameBasket(basket_id)) {
			return;
		} else if (basket_id == -1) {
			baskets[selected_basket_id].item = "";
			players.push(selected_name);
			players = [...players];
			selected_name = null
			selected_basket_id = null
		} else if (selected_basket_id == -1) {
			if (baskets[basket_id].item != "") {
				players.push(baskets[basket_id].item);
			}
			baskets[basket_id].item = selected_name;
			players.splice(players.indexOf(selected_name), 1);
			selected_name = null
			selected_basket_id = null
		} else {
			let copy = selected_name;
			baskets[selected_basket_id].item = baskets[basket_id].item; 
			baskets[basket_id].item = copy; 
			selected_name = null
			selected_basket_id = null
		}
		if (selected_name == baskets[basket_id].item) {
			return;
		}
	}

	function onClickCard(event: any, name: string, basket_id: number) {
		console.log("Card")
		if (selected_name == null) {
			selected_name = name;
			selected_basket_id = basket_id;
		} else if (basket_id == -1) {
			// we cannot do this because we can tell when we click on a card from the basket function
			// baskets[selected_basket_id].item = name;
			// players.push(selected_name)
			// players = [...players];
			// selected_name = null
			// selected_basket_id = null
		}
	}
</script>

{#each baskets as basket, basketIndex (basket)}
	<div>
		{#if localStorage.getItem("game_mode") == "text"}
		<b>{basket.name}</b>
		{:else}
		<img src="{localStorage.getItem("base_server_path")?.replace("api/v1/game/", "")}{geturl(basket.name)}" alt="Shut up"/>
		{/if}
		<ul class="flex-container empty" on:click={(event) => onClickBasket(event, basketIndex)} >
			{#if basket.item != ''}
				<div>
					{#if selected_name == basket.item}
					<li class="highlight" on:click={(event) => onClickCard(event, basket.item, basketIndex)}>
						{basket.item}
					</li>
					{:else}
					<li class="non-highlight" on:click={(event) => onClickCard(event, basket.item, basketIndex)}>
						{basket.item}
					</li>
					{/if}
				</div>
			{/if}
		</ul>
	</div>
{/each}
<div>
<b>{"PLayers"}</b>
</div>
{#if players.length == 0}
	<ul class="flex-container empty" on:click={(event) => onClickBasket(event, -1)}>
		{#each players as item, itemIndex (item)}
			<div>
				{#if selected_name == item}
				<li class="highlight" draggable={true} on:click={(event) => onClickCard(event, item, -1)}>
					{item}
				</li>
				{:else}
				<li class="non-highlight" draggable={true} on:click={(event) => onClickCard(event, item, -1)}>
					{item}
				</li>
				{/if}
			</div>
		{/each}
	</ul>
{:else}
	<ul class="flex-container" on:click={(event) => onClickBasket(event, -1)}>
		{#each players as item, itemIndex (item)}
			<div>
				{#if selected_name == item}
				<li class="highlight" draggable={true} on:click={(event) => onClickCard(event, item, -1)}>
					{item}
				</li>
				{:else}
				<li class="non-highlight" draggable={true} on:click={(event) => onClickCard(event, item, -1)}>
					{item}
				</li>
				{/if}
			</div>
		{/each}
	</ul>
{/if}

<style>
	@import '../app.css';
  	.highlight {
		font-weight: bold;
		background-color: #19beff; /* Background color */
		padding: 10px; /* Padding around content */
		margin: 5px; /* Margin between items */
		font-family: inherit; /* Font family */
		font-size: 16px; /* Font size */
		text-align: center; /* Center text */
		flex: 1; /* Equal width distribution among items */
		border-radius: 5px;
		flex: 0 0 calc(25% - 10px);
	}
  	.non-highlight {
		background-color: #4fcafa; /* Background color */
		padding: 10px; /* Padding around content */
		margin: 5px; /* Margin between items */
		font-family: inherit; /* Font family */
		font-size: 16px; /* Font size */
		text-align: center; /* Center text */
		flex: 1; /* Equal width distribution among items */
		border-radius: 5px;
		flex: 0 0 calc(25% - 10px);
	}
	li:hover {
		font-weight: bold;
		border-radius: 5px;
	}
	li {
		display: inline;
		text-align: left;
	}
	ul {
		color: inherit;
		border: solid rgb(255, 255, 255) 1px;
		padding: 10px;
		list-style-type: none;
		text-align: center;

	}
	@media (prefers-color-scheme: dark) {
		ul {
			color: inherit;
			border: solid rgb(255, 255, 255) 1px;
			padding: 10px;
			list-style-type: none;
			text-align: center;

		}
	}
	@media (prefers-color-scheme: light) {
		ul {
			color: inherit;
			border: solid rgb(0, 0, 0) 1px;
			padding: 10px;
			list-style-type: none;
			text-align: center;

		}
	}
	.empty {
		height: 34px;
	}

</style>
