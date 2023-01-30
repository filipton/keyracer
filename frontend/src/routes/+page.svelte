<script lang="ts">
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';
	import type { InputWord } from '$lib/types';

	let debug: boolean = false;
	let finished: boolean = false;

	async function finishWriting(args: any) {
		let details = args.detail as { time: number; words: InputWord[] };
		finished = true;

		setTimeout(() => {
			let filtered_words = details.words.filter((x) => x.finished);

			// chars count + spaces count
			let correct_chars =
				filtered_words.reduce((total, curr) => (total += curr.characters.length), 0) +
				filtered_words.length;

			let wpm = Math.round(correct_chars / 5 / (details.time / 60000));
			console.log('Calculated WPM: ' + wpm);
		}, 200);
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
			<h1>Keyracer</h1>
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
