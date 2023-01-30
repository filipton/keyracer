<script lang="ts">
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';
	import type { InputWord } from '$lib/types';

	let debug: boolean = false;

	async function finishWriting(args: any) {
		let details = args.detail as { time: number; words: InputWord[] };

		setTimeout(() => {
			let filtered_words = details.words.filter((x) => x.finished);

			// chars count + spaces count
			let correct_chars =
				filtered_words.reduce((total, curr) => (total += curr.characters.length), 0) +
				filtered_words.length;

			let wpm = Math.round(correct_chars / 5 / (details.time / 60000));

			alert('Calculated WPM: ' + wpm);
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
</div>

<div class="main">
	{#await getWordsList(true) then words}
		<KeyracerInput input={words} {debug} on:finished={finishWriting} />
	{/await}
</div>

<style>
	.main {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100vh;
		flex-wrap: wrap;
	}

	.debug-selector {
		position: absolute;
		top: 0;
		right: 0;
		padding: 1rem;
	}
</style>
