<script lang="ts">
	import { page } from '$app/stores';
	import type { NrResult, User } from '$lib/types';
	import { onMount } from 'svelte';

	import { Line } from 'svelte-chartjs';
	import 'chart.js/auto';

	let userInfo: User = $page.data.userInfo;
	let userResults: NrResult[] = $page.data.userResults;
	let chartData: any;

	onMount(() => {
		if (userInfo.id === $page.data.user.id) {
			userInfo = $page.data.user;
		}

		calculate_chart();
	});

	function calculate_chart() {
		if (userResults.length == 0) return;

		chartData = {
			labels: userResults.map((_, i) => i + 1),
			datasets: [
				{
					label: 'WPM',
					lineTension: 0.5,
					borderColor: 'rgb(75, 192, 192)',
					data: userResults.map((x) => x.wpm),
                    yAxisID: 'y'
				},
				{
					label: 'ACCURACY (%)',
					lineTension: 0.5,
					borderColor: 'rgb(192, 75, 192)',
					data: userResults.map((x) => x.acc),
                    yAxisID: 'y'
				},
				{
					label: 'MAX KEYSTROKE TIME (ms)',
					lineTension: 0.5,
					borderColor: 'rgb(192, 192, 75)',
					data: userResults.map((x) => x.max_ks),
                    yAxisID: 'y2'
				}
			]
		};
	}
</script>

<div class="content">
	<h1>{userInfo.name}'s stats</h1>
	<span>Account created: {new Date(Number(userInfo.created_at) * 1000).toLocaleString()}</span>
    <br />

	{#if userInfo.email !== 'REDACTED'}
		<button class="show-email-btn" on:click={() => alert(userInfo.email)}
			>Email: [CLICK TO SHOW]</button
		>
	{/if}
</div>

{#if chartData}
	<h1 style="text-align: center">Last 50 results:</h1>
	<div class="charts-holder">
		<Line
			options={{
				scales: {
					y: {
						beginAtZero: true,
						display: true,
						position: 'left'
					},
					y2: {
						beginAtZero: true,
						display: true,
						position: 'right'
					}
				}
			}}
			data={chartData}
			width={100}
		/>
	</div>
{:else}
	<div class="no-data">
		<h1>NO DATA</h1>
	</div>
{/if}

<style>
	.charts-holder {
		width: 100%;
	}

	.content {
		color: var(--fg-color);
		text-align: center;
		margin-bottom: 4em;
	}

	.show-email-btn {
		background-color: var(--bg-color);
		color: var(--fg-color);
		border: 1px solid var(--fg-color);
		border-radius: 5px;
		padding: 0.5em;
		margin: 0.5em;

		cursor: pointer;
	}

	.no-data {
		position: absolute;
		top: 50%;
		left: 50%;

		transform: translate(-50%, -50%);

		color: gray;
		opacity: 0.5;
		font-size: 2em;
	}
	.no-data > * {
		rotate: 45deg;
	}
</style>
