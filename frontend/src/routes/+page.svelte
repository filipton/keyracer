<script lang="ts">
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';
	import KeyracerStats from '$lib/components/KeyracerStats.svelte';
	import type { KeyracerFinishDetails, KeyracerResponse, QuoteJson } from '$lib/types';

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

		await fetch('http://localhost:8080/response', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
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
			const response = await fetch('http://localhost:8080/words/15');
			const words = await response.json();
			return words.join(' ');
		}

		const response = await fetch('http://localhost:8080/quote');
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

<div class="main">
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
	.main {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-wrap: wrap;

		height: calc(100vh - 59px);

		margin-left: auto;
		margin-right: auto;
		max-width: 960px;

		animation: fadeIn 1.5s;
		animation-fill-mode: forwards;
		opacity: 0;
	}
	@keyframes fadeIn {
		0% {
			opacity: 0;
		}
		100% {
			opacity: 1;
		}
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
