<script lang="ts">
	import { page } from '$app/stores';
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';
	import KeyracerStats from '$lib/components/KeyracerStats.svelte';
	import {
		apiUrl,
		type KeyracerFinishDetails,
		type RankedQuote,
		type RankedResponse,
		type RankingEntry
	} from '$lib/types';
	import { onMount } from 'svelte';

	$: rankedAvailable = $page.data.rankedAvailable as boolean;
	let rankedQuote: string;
	let rankedQuoteId: number = -1;

	let ranking: RankingEntry[];

	let finishedDetails: KeyracerFinishDetails;
	let finished: boolean = false;

	let errorMessage: string = ' ';
	let errorTimeout: NodeJS.Timeout;

	onMount(async () => {
		await fetchRanking();

		if (!rankedAvailable) {
			setError("You can't play ranked right now.", true);
		}
	});

	async function Participate() {
		if (!$page.data.user) {
			setError('You need to be logged in to participate in a ranked match.');
			return;
		}

		const res = await fetch(`${apiUrl}/ranked/start`, {
			method: 'POST',
			headers: {
				Auth: $page.data.token
			}
		});
		if (res.ok) {
			const data: RankedQuote = await res.json();
			rankedQuote = data.quote;
			rankedQuoteId = data.id;
		} else if (res.status === 403) {
			setError("You've already participated in this day!");
		} else {
			setError('Something went wrong.');
		}
	}

	async function finishWriting(args: any) {
		finishedDetails = args.detail;
		finished = true;

		let keystrokesStr: string = finishedDetails.history
			.map((x) => `${x.time}><${x.input}`)
			.join('\n');
		let data: RankedResponse = {
			time: finishedDetails.time,
			quote_id: rankedQuoteId,
			chars_written: finishedDetails.charsWritten,
			chars_correct: finishedDetails.charsCorrect,
			chars_in_correct_words: finishedDetails.charsInCorrectWords,
			history: keystrokesStr
		};

		await fetch(`${apiUrl}/ranked/submit`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Auth: $page.data.token
			},
			body: JSON.stringify(data)
		}).then(async (x: Response) => {
			if (!x.ok) {
				alert('Error');
			}
		});
	}

	async function fetchRanking() {
		await fetch(`${apiUrl}/ranked/ranking`)
			.then((x: Response) => x.json())
			.then((x: RankingEntry[]) => {
				ranking = x;
			});
	}

	function setError(msg: string, forever: boolean = false) {
		clearTimeout(errorTimeout);
		errorMessage = msg;

		if (forever) return;
		errorTimeout = setTimeout(() => {
			errorMessage = '';
		}, 5000);
	}
</script>

<div class="container">
	{#if !$page.data.user}
		<a href="/user" class="btn">YOU MUST BE LOGGED IN!</a>
	{:else if rankedQuote}
		{#if finished}
			<div class="main-screen">
				<h2>STATS</h2>
				<KeyracerStats details={finishedDetails} />

				<a href="/" style="margin-top: auto; margin-bottom: 10px;" class="btn">BACK TO MAIN MENU</a>
			</div>
		{:else}
			<KeyracerInput input={rankedQuote} debug={false} on:finished={finishWriting} />
		{/if}
	{:else}
		<span class="h1">Ranked</span>
		<span class="h2">Daily (refreshes at 00:00 UTC)</span>

		<div style="margin-top: 3em; text-align: center;">
			<div class="error">{errorMessage}</div>

			<button on:click={Participate} class="btn {rankedAvailable ? '' : 'disabled'}"
				>PARTICIPATE</button
			>
		</div>
	{/if}

	{#if ranking && !rankedQuote}
		<h1>Ranking</h1>

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

<style>
	.container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;

		width: 100%;
		height: 100%;
	}

	.disabled {
		opacity: 0.5;
		pointer-events: none;
	}

	.main-screen {
		display: flex;
		flex-direction: column;
		align-items: center;

		height: 100%;
		width: 100%;
	}

	.error {
		color: red;
	}

	.h1 {
		font-size: 2rem;
	}
	.h2 {
		font-size: 1.5rem;
	}

	table {
		border-collapse: collapse;
	}

	th,
	td {
		padding-left: 0.5em;
		padding-right: 0.5em;

		border: 1px solid var(--fg-color);
	}
</style>
