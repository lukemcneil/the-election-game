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

	function getCardClass(select_name, item) {
		if (select_name == item) {
			return "highlight"
		} else {
			return "non-highlight"
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
		<ul class="shadow holder" on:click={(event) => onClickBasket(event, basketIndex)} >
			{#if basket.item != ''}
				<div>
					<li class="shadow card {getCardClass(selected_name, basket.item)}" on:click={(event) => onClickCard(event, basket.item, basketIndex)}>
						{basket.item}
					</li>
				</div>
			{/if}
		</ul>
	</div>
{/each}
<div>
<b>{"Players"}</b>
</div>
<ul class="flex-container shadow bank" on:click={(event) => onClickBasket(event, -1)}>
	{#each players as item, itemIndex (item)}
		<div>
			<li class="shadow card {getCardClass(selected_name, item)}" draggable={true} on:click={(event) => onClickCard(event, item, -1)}>
				{item}
			</li>
		</div>
	{/each}
</ul>

<style>
	@import '../app.css';
	.holder {
		display: table;
		margin-inline: auto;
		padding: 0.01px;
		border-radius: 5px;
		min-width: 80px;
		min-height: 55px;
	}
	.card {
		font-size: 16px;
		font-family: inherit;
		text-align: center;
		border-radius: 5px;
		text-align: center;
		font-family: inherit;
		font-size: 16px;
		padding: 10px;
	}
	.bank {
		min-height: 55px;
	}
  	.highlight {
		background-color: #4fcafa;
	}
  	.non-highlight {
		background-color: #387b96;
	}
	li:hover {
		background-color: #4fcafa;
	}
	ul {
		list-style-type: none;
	}
	@media (prefers-color-scheme: dark) {
		ul {
			background-color: rgba(255,255,255,0.15);
		}
	}
	@media (prefers-color-scheme: light) {
		ul {
			background-color: rgba(0,0,0,0.15);
		}
	}
	.empty {
		height: 55px;
	}

</style>
