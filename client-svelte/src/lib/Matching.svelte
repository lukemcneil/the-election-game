<script src="DragDropTouch.js" lang="ts">
	import { flip } from 'svelte/animate';

	export let baskets: Array<{ name: string; item: string }> = [];
	export let players: Array<string> = [];

	let hoveringOverBasket: string | null;

	function dragStart(event, basketIndex: number, item: string | number) {
		let data;
		let itemName;
		if (basketIndex == -1) {
			itemName = players[item];
			data = { basketIndex, itemName };
		} else {
			itemName = item;
			data = { basketIndex, itemName };
		}
		event.dataTransfer.setData('text/plain', JSON.stringify(data));
	}

	function drop(event, drop_basket_id: number) {
		event.preventDefault();
		const json = event.dataTransfer.getData('text/plain');
		const data = JSON.parse(json);

		if (drop_basket_id == -1) {
			if (data.basketIndex != -1) {
				players = [...players, data.itemName];
				baskets[data.basketIndex].item = '';
			}
		} else if (baskets[drop_basket_id].item == '') {
			if (data.basketIndex == -1) {
				players.splice(players.indexOf(data.itemName), 1);
				players = [...players];
				baskets[drop_basket_id].item = data.itemName;
			} else {
				console.log(baskets[drop_basket_id].item);
				baskets[data.basketIndex].item = '';
				baskets[drop_basket_id].item = data.itemName;
			}
		} else {
			if (data.basketIndex == -1) {
				players.splice(players.indexOf(data.itemName), 1);
				players = [...players, baskets[drop_basket_id].item];
				baskets[drop_basket_id].item = data.itemName;
			} else {
				baskets[data.basketIndex].item = baskets[drop_basket_id].item;
				baskets[drop_basket_id].item = data.itemName;
			}
		}

		hoveringOverBasket = null;
	}
</script>

{#each baskets as basket, basketIndex (basket)}
	<div animate:flip>
		<b>{basket.name}</b>
		<ul
			class:hovering={hoveringOverBasket === basket.name}
			on:dragenter={() => (hoveringOverBasket = basket.name)}
			on:dragleave={() => (hoveringOverBasket = null)}
			on:drop={(event) => drop(event, basketIndex)}
			ondragover="return false"
		>
			{#if basket.item != ''}
				<div class="item">
					<li draggable={true} on:dragstart={(event) => dragStart(event, basketIndex, basket.item)}>
						{basket.item}
					</li>
				</div>
			{/if}
		</ul>
	</div>
{/each}
<div>Players</div>
<ul
	class:hovering={hoveringOverBasket === 'bank'}
	on:dragenter={() => (hoveringOverBasket = 'bank')}
	on:dragleave={() => (hoveringOverBasket = null)}
	on:drop={(event) => drop(event, -1)}
	ondragover="return false"
>
	{#each players as item, itemIndex (item)}
		<div class="item" animate:flip>
			<li draggable={true} on:dragstart={(event) => dragStart(event, -1, itemIndex)}>
				{item}
			</li>
		</div>
	{/each}
</ul>

<style>
	/* @import '../app.css'; */
  div {
    text-align: center;
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
