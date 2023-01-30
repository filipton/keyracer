<script lang="ts">
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';

	let debug: boolean = false;

	async function finishWriting(args: any) {
        let details = args.detail as { time: number, words_count: number };

        alert("Calculated WPM: " + Math.round((details.words_count / (details.time / 1000)) * 60));
	}

	async function getWordsList() {
		const response = await fetch('http://localhost:8080/words/15');
		const words = await response.json();
		return words.join(' ');
	}
</script>

<div class="debug-selector">
	<label for="debug">Debug</label>
	<input type="checkbox" id="debug" bind:checked={debug} />
</div>

<div class="main">
	{#await getWordsList() then words}
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
