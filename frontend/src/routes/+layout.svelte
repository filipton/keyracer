<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import SettingsMenu from '$lib/components/SettingsMenu.svelte';
	import { settingsMenuActive } from '$lib/stores';

	async function onKeyDown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			if (!$settingsMenuActive) {
				settingsMenuActive.set(true);
				return;
			}
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

{#if $settingsMenuActive}
	<SettingsMenu
		on:close={() => {
			settingsMenuActive.set(false);
		}}
	/>
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
