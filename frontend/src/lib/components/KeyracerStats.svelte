<script lang="ts">
	import { CharState, type KeyracerFinishDetails } from '$lib/types';
	import { onMount } from 'svelte';

	import { Line } from 'svelte-chartjs';
	import 'chart.js/auto';

	export let details: KeyracerFinishDetails;

	let wpm: number = 0;
	let rawWpm: number = 0;
	let time: string = '0.00s';
	let accuracy: string = '0%';

	let data: any;

	onMount(() => {
		if (!details) return;

		calculate();
		calculate_chart();
	});

	function calculate() {
		wpm = Math.round(details.charsInCorrectWords / 5 / (details.time / 60000));
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
		rawWpm = Math.max(rawWpm, wpm);
		accuracy = `${Math.round((details.charsCorrect / details.charsWritten) * 100)}%`;
	}

	function calculate_chart() {
		let datas = details.history.map((x, i) => {
			let _time = x.time - (details.history[i - 1]?.time ?? 0);
			return _time / 1000;
		});

		data = {
			labels: details.history.map((_, i) => i),
			datasets: [
				{
					label: 'Keystroke Time (s)',
					fill: true,
					lineTension: 0.5,
					borderColor: window.getComputedStyle(document.body).getPropertyValue('--fg-color'),
					data: datas
				}
			]
		};
	}
</script>

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

{#if data}
	<div class="charts-holder">
		<Line {data} width={100} />
	</div>
{/if}

<style>
	.infos-holder {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1em;
	}

	.charts-holder {
		margin-top: 3em;
		width: 100%;
	}

	.infos-holder > .info-box {
		border: 1px solid var(--fg-color);
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
</style>
