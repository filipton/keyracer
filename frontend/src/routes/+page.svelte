<script lang="ts">
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';

	let debug: boolean = false;
</script>

<label for="debug">Debug</label>
<input type="checkbox" id="debug" bind:checked={debug} />

<br />
<br />
<br />

<KeyracerInput input="this is a test string to write" {debug} />
