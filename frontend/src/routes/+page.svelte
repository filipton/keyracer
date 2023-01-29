<script lang="ts">
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';

	let debug: boolean = false;

	async function finishWriting() {
		document.body.style.backgroundColor = 'green';
	}
</script>

<div class="debug-selector">
	<label for="debug">Debug</label>
	<input type="checkbox" id="debug" bind:checked={debug} />
</div>

<div class="main">
	<KeyracerInput
        input="tank head table boundless door crayon hulking important help highfalutin fragile lovely minor cooing marvelous"
		{debug}
		on:finished={finishWriting}
	/>
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
