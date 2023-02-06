<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import SettingsMenu from '$lib/components/SettingsMenu.svelte';
	import { settingsMenuActive } from '$lib/stores';

	let settingsMenuOpen: boolean = false;

	async function onKeyDown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			settingsMenuOpen = !settingsMenuOpen;
            settingsMenuActive.set(settingsMenuOpen);
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

{#if settingsMenuOpen}
	<SettingsMenu on:close={() => {
        settingsMenuOpen = false;
        settingsMenuActive.set(settingsMenuOpen);
    }} />
{/if}

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
