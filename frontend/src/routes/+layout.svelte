<script>
	import Header from '$lib/components/Header.svelte';
</script>

<div class="main">
	<Header />
	<slot />
</div>

<style>
	.main {
		margin-left: auto;
		margin-right: auto;
		padding-left: 1em;
		padding-right: 1em;
		max-width: 960px;

		animation: fadeIn 1.5s;
		animation-fill-mode: forwards;
		opacity: 0;

		height: calc(100% - 59px);
	}
	@keyframes fadeIn {
		0% {
			opacity: 0;
		}
		100% {
			opacity: 1;
		}
	}
</style>
