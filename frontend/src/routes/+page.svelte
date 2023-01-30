<script lang="ts">
	import KeyracerInput from '$lib/components/KeyracerInput.svelte';
	import { CharState, type InputWord } from '$lib/types';

	let debug: boolean = false;

	let wpm: number = 0;
	let rawWpm: number = 0;
	let time: string = '0.00s';
	let accuracy: string = '0%';
	let finished: boolean = false;

	async function finishWriting(args: any) {
		let details = args.detail as {
			time: number;
			words: InputWord[];
			charsWritten: number;
			charsCorrect: number;
		};
		finished = true;

		let filtered_words = details.words.filter((x) => x.finished);
		let correct_chars =
			filtered_words.reduce((total, curr) => (total += curr.characters.length), 0) +
			filtered_words.length;

		wpm = Math.round(correct_chars / 5 / (details.time / 60000));
		time = `${Math.round(details.time / 10) / 100}s`;

		let raw_chars =
			details.words.reduce(
				(total, curr) =>
					(total += curr.characters.filter((x) => x.state !== CharState.NotStarted).length),
				0
			) +
			details.words.length -
			1;
		rawWpm = Math.round(raw_chars / 5 / (details.time / 60000));

		console.log(details.charsWritten, details.charsCorrect);
		accuracy = `${Math.round((details.charsCorrect / details.charsWritten) * 100)}%`;
	}

	async function getWordsList(quote: boolean = false) {
		if (!quote) {
			const response = await fetch('http://localhost:8080/words/15');
			const words = await response.json();
			return words.join(' ');
		}

		const response = await fetch('http://localhost:8080/quote');
		const qresp = (await response.json()) as { quote: string; author: string };
		return qresp.quote;
	}
</script>

<div class="debug-selector">
	<label for="debug">Debug</label>
	<input type="checkbox" id="debug" bind:checked={debug} />

	<label for="finished">Finished</label>
	<input type="checkbox" id="finished" bind:checked={finished} />
</div>

<div class="main">
	{#if finished}
		<div class="main-screen">
			<h1>KEYRACER</h1>

			<h2>STATS</h2>
			<div class="infos-holder">
				<div class="info-box">
					<h2>WPM</h2>
					<h3>{wpm}</h3>
				</div>
				<div class="info-box">
					<h2>RAW</h2>
					<h3>{rawWpm}</h3>
				</div>
				<div class="info-box">
					<h2>TIME</h2>
					<h3>{time}</h3>
				</div>
				<div class="info-box">
					<h2>ACC</h2>
					<h3>{accuracy}</h3>
				</div>
			</div>

			<button
				style="margin-top: auto; margin-bottom: 10px;"
				class="btn"
				on:click={() => {
					finished = false;
				}}>RESTART</button
			>
		</div>
	{:else}
		{#await getWordsList(true) then words}
			<KeyracerInput input={words} {debug} on:finished={finishWriting} />
		{/await}
	{/if}
</div>

<style>
	.btn {
		background-color: transparent;
		border: 3px solid black;
		border-radius: 5px;
		color: black;
		padding: 15px 32px;
		text-align: center;
		text-decoration: none;
		display: inline-block;
		font-size: 1em;
		margin: 4px 2px;
		cursor: pointer;
	}

	.main {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100vh;
		flex-wrap: wrap;
	}
	.main > * {
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

		height: 100vh;
		width: 100vw;
	}

	.infos-holder {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1em;
	}

	.infos-holder > .info-box {
		border: 1px solid black;
		border-radius: 5px;
		padding: 10px;

		width: 4em;
		height: 4em;

		line-height: 0;

		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.debug-selector {
		position: absolute;
		top: 0;
		right: 0;
		padding: 1rem;
		z-index: 100;
	}
</style>
