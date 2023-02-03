<script lang="ts">
	import { page } from '$app/stores';
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';
	import KeyracerStats from '$lib/components/KeyracerStats.svelte';
	import {
		apiUrl,
		type KeyracerFinishDetails,
		type KeyracerResponse,
		type QuoteJson
	} from '$lib/types';

	let debug: boolean = false;
	let finished: boolean = false;
	let selectedQuotes: boolean = false;

	let finishedDetails: KeyracerFinishDetails;

	async function finishWriting(args: any) {
		finishedDetails = args.detail;
		finished = true;

		let keystrokesStr: string = finishedDetails.history
			.map((x) => `${x.time}><${x.input}`)
			.join('\n');
		let data: KeyracerResponse = {
			time: finishedDetails.time,
			chars_written: finishedDetails.charsWritten,
			chars_correct: finishedDetails.charsCorrect,
			chars_in_correct_words: finishedDetails.charsInCorrectWords,
			history: keystrokesStr
		};

		if (!$page.data.token) {
			setTimeout(() => alert('Login to save your results!'), 1000);
			return;
		}

		await fetch(`${apiUrl}/results`, {
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

	async function getWordsList(quote: boolean = false): Promise<string> {
		if (!quote) {
			const response = await fetch(`${apiUrl}/test/words/15`);
			const words = await response.json();
			return words.join(' ');
		}

		const response = await fetch(`${apiUrl}/test/quote`);
		const qresp: QuoteJson = await response.json();
		return qresp.quote;
	}

	async function onKeyDown(event: KeyboardEvent) {
		if (event.key === '~' && event.ctrlKey) {
			event.preventDefault();
			debug = !debug;
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

{#if debug}
	<div class="debug-selector">
		<h1 style="text-align: center; line-height: 0;">Debug</h1>

		<label for="quotes">Quotes</label>
		<input type="checkbox" id="quotes" bind:checked={selectedQuotes} />

		<label for="finished">Finished</label>
		<input type="checkbox" id="finished" bind:checked={finished} />
	</div>
{/if}

<div class="container">
	{#if finished}
		<div class="main-screen">
			<h2>STATS</h2>
			<KeyracerStats details={finishedDetails} />

			<button
				style="margin-top: auto; margin-bottom: 10px;"
				class="btn"
				on:click={() => {
					finished = false;
				}}>RESTART</button
			>
		</div>
	{:else}
		{#await getWordsList(selectedQuotes) then words}
			<KeyracerInput input={words} {debug} on:finished={finishWriting} />
		{/await}
	{/if}
</div>

<style>
	.container {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-wrap: wrap;

		width: 100%;
		height: 100%;
	}

	.main-screen {
		display: flex;
		flex-direction: column;
		align-items: center;

		height: 100%;
		width: 100%;
	}

	.debug-selector {
		position: absolute;
		transform: translate(50%, 0%);

		top: 0;
		right: 50%;
		padding: 0rem 1rem 1rem 1rem;
		z-index: 100;

		background-color: var(--bg-color);
		border: 1px solid var(--fg-color);
		border-top: none;
	}
</style>
