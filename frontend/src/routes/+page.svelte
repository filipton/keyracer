<script lang="ts">
	import { page } from '$app/stores';
	import { wordsList, quotesList, settings } from '$lib/stores';
	import KeyracerInput from '$lib/components/Input/KeyracerInput.svelte';
	import KeyracerStats from '$lib/components/Input/KeyracerStats.svelte';
	import {
		apiUrl,
		Mode,
		type KeyracerFinishDetails,
		type KeyracerResponse,
		type QuoteJson
	} from '$lib/types';

	let debug: boolean = false;
	let finished: boolean = false;
	let restartButtonShown: boolean = false;

	let input: string;

	let restartButton: HTMLElement;
	let finishedDetails: KeyracerFinishDetails;

	settings.subscribe(async (x) => {
		if (!x) return;

		await getWordsList();
		await getInput();
	});

	async function getInput() {
		switch ($settings.mode) {
			case Mode.Words:
				let wordsCount = 15;
				input = $wordsList
					.sort(() => Math.random() - 0.5)
					.slice(0, wordsCount)
					.join(' ');
				break;

			case Mode.Quote:
				let quote = $quotesList[Math.floor(Math.random() * $quotesList.length)];
				input = quote.quote;
				break;

			default:
				input = 'ErRoR dUe tO iNvAlId MoDe';
				break;
		}
	}

	async function finishWriting(args: any) {
		finishedDetails = args.detail;
		finished = true;
		restartButtonShown = true;

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

		if (!$page.data.user || !$page.data.token) return;
		await fetch(`${apiUrl}/results`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Auth: $page.data.token
			},
			body: JSON.stringify(data)
		}).then(async (x: Response) => {
			if (!x.ok) {
				console.error('Error while saving results');
			}
		});
	}

	async function getWordsList() {
		if ($wordsList.length > 0 && $quotesList.length > 0) return;

		await Promise.all([
			fetch(`${apiUrl}/data/words`).then(async (x: Response) => {
				wordsList.set((await x.text()).split('\n'));
			}),
			fetch(`${apiUrl}/data/quotes`).then(async (x: Response) => {
				quotesList.set((await x.json()) as QuoteJson[]);
			})
		]);
	}

	async function restart() {
		if (!restartButtonShown) return;

		finished = false;
		restartButtonShown = false;

		await getInput();
	}

	async function onKeyDown(event: KeyboardEvent) {
		if (event.key === '~' && event.ctrlKey) {
			event.preventDefault();
			debug = !debug;
		}

		if (event.key === 'Tab') {
			event.preventDefault();
			restartButton.focus();
			restartButtonShown = true;
        } else if(event.key === " " && restartButtonShown) {
            await restart();
		} else if (event.key !== 'Enter' && !finished) {
			restartButtonShown = false;
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

{#if debug}
	<div class="debug-selector">
		<h1 style="text-align: center; line-height: 0;">Debug</h1>

		<label for="finished">Finished</label>
		<input type="checkbox" id="finished" bind:checked={finished} />
	</div>
{/if}

<div class="container">
	{#if finished}
		<div class="main-screen">
			<h2>STATS</h2>
			<KeyracerStats details={finishedDetails} />
		</div>
	{:else if input}
		<KeyracerInput {input} {debug} on:finished={finishWriting} />
	{/if}

	<button
		style="{restartButtonShown ? 'opacity: 1;' : 'opacity: 0;'}}"
		class="btn restartButton"
		on:click={restart}
		bind:this={restartButton}
		><span class="btn-tooltip">ENTER</span>
		RESTART</button
	>
</div>

<style>
	.restartButton {
		position: absolute;
		bottom: 10px;

		opacity: 0;
		transition: opacity 0.5s;
	}

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
