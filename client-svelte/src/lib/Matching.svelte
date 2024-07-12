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

	function onClickBasket(event: any, basket_id: number) {
		console.log("Basket")
		if (selected_name == baskets[basket_id].item) {
			return;
		}
		if (selected_name != null) {
			// console.log(basket_id)
			if (basket_id == -1) {
				baskets[selected_basket_id].item = "";
				players.push(selected_name);
				players = [...players];
				selected_name = null
				selected_basket_id = null
			} else if (selected_basket_id == -1) {
				baskets[basket_id].item = selected_name;
				players.splice(players.indexOf(selected_name), 1);
				players = [...players];
				selected_name = null
				selected_basket_id = null
			} else {

			}
		}
	}

	function onClickCard(event: any, name: string, basket_id: number) {
		console.log("Card")
		if (selected_name == null) {
			// console.log("here")
			selected_name = name;
			selected_basket_id = basket_id;
		} else {
			if (selected_basket_id == -1) {
				players.splice(players.indexOf(selected_name), 1);
				players.push(name);
				players = [...players];
				baskets[basket_id].item = name;
				selected_name = null
				selected_basket_id = null
			} else if (basket_id == -1) {
				baskets[basket_id].item = "";
				players.push(name);
				players = [...players];
				selected_name = null
				selected_basket_id = null
			} else {
				let copy = selected_name;
				baskets[selected_basket_id].item = name; 
				baskets[basket_id].item = copy; 
				selected_name = null
				selected_basket_id = null
			}
		}
	}
</script>

{#each baskets as basket, basketIndex (basket)}
	<div animate:flip>
		{#if localStorage.getItem("game_mode") == "text"}
		<b>{basket.name}</b>
		{:else}
		<img src="{localStorage.getItem("base_server_path")?.replace("api/v1/game/", "")}{geturl(basket.name)}" alt="Shut up"/>
		{/if}
		<ul on:click={(event) => onClickBasket(event, basketIndex)} >
			{#if basket.item != ''}
				<div class="item">
					{#if selected_name == basket.item}
					<li id="highlight" on:click={(event) => onClickCard(event, basket.item, basketIndex)}>
						{basket.item}
					</li>
					{:else}
					<li on:click={(event) => onClickCard(event, basket.item, basketIndex)}>
						{basket.item}
					</li>
					{/if}
				</div>
			{/if}
		</ul>
	</div>
{/each}
<div>Players</div>
<ul on:click={(event) => onClickBasket(event, -1)}>
	{#each players as item, itemIndex (item)}
		<div class="item" animate:flip>
			{#if selected_name == item}
			<li id="highlight" draggable={true} on:click={(event) => onClickCard(event, item, -1)}>
				{item}
			</li>
			{:else}
			<li draggable={true} on:click={(event) => onClickCard(event, item, -1)}>
				{item}
			</li>
			{/if}
		</div>
	{/each}
</ul>

<style>
	/* @import '../app.css'; */
  div {
    text-align: center;
  }
  	#highlight {
		background: cornflowerblue;
		color: white;
	}
	.hovering {
		border-color: cornflowerblue;
	}
	.item {
		display: inline; /* required for flip to work */
	}
	li {
		background-color: lightgray;
		cursor: pointer;
		display: inline-block;
		margin-right: 10px;
		padding: 10px;
	}
	li:hover {
		background: cornflowerblue;
		color: white;
	}
	ul {
		border: solid lightgray 1px;
		display: flex; /* required for drag & drop to work when .item display is inline */
		height: 40px; /* needed when empty */
		padding: 10px;
	}
</style>
