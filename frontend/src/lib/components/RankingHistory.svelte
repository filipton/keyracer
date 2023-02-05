<script lang="ts">
	import { apiUrl, type RankingEntry, type RankingHistoryEntry } from '$lib/types';
	import { createEventDispatcher, onMount } from 'svelte';

	export let rankingHistory: RankingHistoryEntry[];
	let dispatch = createEventDispatcher();

	let selectedId: number = rankingHistory.at(0)?.id ?? -1;
	let ranking: RankingEntry[] = [];

	onMount(async () => {
		await fetchRanking(selectedId);
	});

	async function buttonClick(mv: number) {
		if (selectedId === -1) return;

		const newIndex = rankingHistory.findIndex((e) => e.id === selectedId) - mv;
		if (newIndex < 0 || newIndex >= rankingHistory.length) return;

		selectedId = rankingHistory.at(newIndex)?.id ?? -1;
		await fetchRanking(selectedId);
	}

	function infoButtonClick() {
		alert('Quote from that day: ' + rankingHistory.find((e) => e.id === selectedId)?.quote);
	}

	async function fetchRanking(id: number) {
		if (id === -1) return;

		const res = await fetch(`${apiUrl}/ranked/ranking/${id}`);
		if (res.ok) {
			ranking = await res.json();
			console.log(ranking);
		} else {
			console.error('Something went wrong.');
		}
	}
</script>

<div class="window">
	<h1 class="centered-h1">Ranking</h1>

	<div class="nav">
		<button on:click={async () => buttonClick(-1)}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="2"
				stroke="currentColor"
				style="width: 48px; height: 48px;"
			>
				<path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5" />
			</svg>
		</button>

		<select bind:value={selectedId} on:change={() => fetchRanking(selectedId)}>
			{#each rankingHistory as entry}
				<option value={entry.id}
					>{new Date(Number(entry.start_at) * 1000).toLocaleString('pl-PL')}</option
				>
			{/each}
		</select>

		<button on:click={async () => buttonClick(1)}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="2"
				stroke="currentColor"
				style="width: 48px; height: 48px;"
			>
				<path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
			</svg>
		</button>
	</div>

	<div>
		{#if ranking}
			<table>
				<tr>
					<th>#</th>
					<th>Name</th>
					<th>WPM</th>
				</tr>

				{#each ranking as entry, i}
					<tr>
						<td>{i + 1}</td>
						<td>{entry.name}</td>
						<td>{entry.wpm.toPrecision(4)}</td>
					</tr>
				{/each}
			</table>
		{/if}
	</div>

	<button class="btn close" on:click={() => dispatch('close')}>Close</button>
	<button class="btn info" on:click={infoButtonClick}>?</button>
</div>

<style>
	.window {
		position: absolute;

		/*
            2em - (margin you want) * 2
            4px - border * 2
        */
		width: calc(100vw - 2em - 4px);
		height: calc(100vh - 2em - 4px);
		max-width: 600px;

		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);

		z-index: 1000;

		border-radius: 0.5em;
		background-color: var(--bg-color);
		border: 2px solid var(--fg-color);
	}

	.close {
		position: absolute;
		bottom: 0.5em;
		left: 50%;

		transform: translateX(-50%);
	}

	.info {
		position: absolute;
		bottom: 0.5em;
		right: 0.5em;

		padding-left: 0.5em;
		padding-right: 0.5em;
		padding-top: 0.25em;
		padding-bottom: 0.25em;
	}

	.nav {
		display: flex;
		justify-content: space-between;

		margin: 0.5em;
	}
	.nav > button {
		border: none;
		background-color: var(--bg-color);
		color: var(--fg-color);
	}

	.nav > select {
		border: 1px solid var(--fg-color);

		background-color: var(--bg-color);
		color: var(--fg-color);
		width: 100%;
		text-align: center;
		font-size: 1rem;

		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
	}

	.centered-h1 {
		font-size: 2rem;
		text-align: center;
		margin: 0;
	}

	table {
		border-collapse: collapse;
		width: 100%;

		text-align: center;
	}

	th,
	td {
		padding-left: 0.5em;
		padding-right: 0.5em;

		border: 1px solid var(--fg-color);
	}

	tr > th {
		padding: 0.5em;
	}

	tr > th:nth-child(1) {
		width: 10%;
	}

	tr > th:nth-child(2) {
		width: 60%;
	}

	tr > th:nth-child(3) {
		width: 30%;
	}
</style>
