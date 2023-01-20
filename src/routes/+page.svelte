<script lang="ts">
	import { onMount } from 'svelte';
	import init from 'wasm';

	onMount(async () => {
		await init();
	});
</script>

<h1>Conway's game of life using SvelteKit + WASM in Rust</h1>

<div id="game" />

<style>
	#game {
		display: grid;
		width: fit-content;
		grid-template-columns: repeat(20, 1fr);
		grid-template-rows: repeat(20, 1fr);
	}

	:global(.cell) {
		border: 4px solid white;
		width: 15px;
		height: 15px;
	}
</style>
