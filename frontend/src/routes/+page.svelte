<script lang="ts">
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';
	import KeyracerStats from '$lib/components/KeyracerStats.svelte';
	import type { KeyracerFinishDetails } from '$lib/types';

	let debug: boolean = false;
	let finished: boolean = false;

	let finishedDetails: KeyracerFinishDetails;

	async function finishWriting(args: any) {
		finishedDetails = args.detail;
		finished = true;
	}

	async function getWordsList(quote: boolean = false) {
		if (!quote) {
			const response = await fetch('http://localhost:8080/words/15');
			const words = await response.json();
			return words.join(' ');
		}

		const response = await fetch('http://localhost:8080/quote');
		const qresp = (await response.json()) as { quote: string; author: string };
		return qresp.quote;
	}
</script>

<div class="debug-selector">
	<label for="debug">Debug</label>
	<input type="checkbox" id="debug" bind:checked={debug} />

	<label for="finished">Finished</label>
	<input type="checkbox" id="finished" bind:checked={finished} />
</div>

<div class="main">
	{#if finished}
		<div class="main-screen">
			<h1>KEYRACER</h1>

			<h2>STATS</h2>
			<KeyracerStats details={finishedDetails} />

			<button
				style="margin-top: auto; margin-bottom: 10px;"
				class="btn"
				on:click={() => {
					finished = false;
				}}>RESTART</button
			>
		</div>
	{:else}
		{#await getWordsList(true) then words}
			<KeyracerInput input={words} {debug} on:finished={finishWriting} />
		{/await}
	{/if}
</div>

<style>
	.main {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100vh;
		flex-wrap: wrap;
	}
	.main > * {
		animation: fadeIn 1.5s;
		animation-fill-mode: forwards;
		opacity: 0;
	}
	@keyframes fadeIn {
		0% {
			opacity: 0;
		}
		100% {
			opacity: 1;
		}
	}

	.main-screen {
		display: flex;
		flex-direction: column;
		align-items: center;

		height: 100vh;
		width: 100vw;
	}

	.debug-selector {
		position: absolute;
		top: 0;
		right: 0;
		padding: 1rem;
		z-index: 100;
	}
</style>
