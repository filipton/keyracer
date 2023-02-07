<script lang="ts">
	import { getCookie, setCookie } from '$lib/utils';
	import { createEventDispatcher, onMount } from 'svelte';

	let dispatch = createEventDispatcher();

	let bgColor: string = '#000000';
	let fgColor: string = '#ffffff';
	let lCorrectColor: string = '#ffffff';
	let lNsColor: string = 'gray';
	let lIncorrectColor: string = '#ff0000';
	let lExtraColor: string = 'darkred';
	let lIncorrectUnderline: string = 'red';
	let lCaretColor: string = 'green';

	onMount(async () => {
		let themeCookie = getCookie('theme');
		if (themeCookie == null) return;

		let customTheme = themeCookie.split(',');

		bgColor = customTheme[0];
		fgColor = customTheme[1];
		lCorrectColor = customTheme[2];
		lNsColor = customTheme[3];
		lIncorrectColor = customTheme[4];
		lExtraColor = customTheme[5];
		lIncorrectUnderline = customTheme[6];
		lCaretColor = customTheme[7];
	});

	function updateTheme(changeCookie: boolean = false) {
		let customTheme = `${bgColor},${fgColor},${lCorrectColor},${lNsColor},${lIncorrectColor},${lExtraColor},${lIncorrectUnderline},${lCaretColor}`;
		if (changeCookie) setCookie('theme', customTheme, 365);

		document.documentElement.style.setProperty('--bg-color', bgColor);
		document.documentElement.style.setProperty('--fg-color', fgColor);
		document.documentElement.style.setProperty('--l-correct-color', lCorrectColor);
		document.documentElement.style.setProperty('--l-ns-color', lNsColor);
		document.documentElement.style.setProperty('--l-incorrect-color', lIncorrectColor);
		document.documentElement.style.setProperty('--l-extra-color', lExtraColor);
		document.documentElement.style.setProperty('--l-incorrect-underline', lIncorrectUnderline);
		document.documentElement.style.setProperty('--l-caret-color', lCaretColor);
	}
</script>

<div class="floating-window">
	<div class="grid-container">
		<div>
			<label for="bg-color"> Background color: </label>
			<input type="color" id="bg-color" bind:value={bgColor} on:change={() => updateTheme()} />
		</div>

		<div>
			<label for="fg-color"> Foreground color: </label>
			<input type="color" id="fg-color" bind:value={fgColor} on:change={() => updateTheme()} />
		</div>

		<div>
			<label for="l-correct-color"> Correct color: </label>
			<input
				type="color"
				id="l-correct-color"
				bind:value={lCorrectColor}
				on:change={() => updateTheme()}
			/>
		</div>

		<div>
			<label for="l-ns-color"> Not started color: </label>
			<input type="color" id="l-ns-color" bind:value={lNsColor} on:change={() => updateTheme()} />
		</div>

		<div>
			<label for="l-incorrect-color"> Incorrect color: </label>
			<input
				type="color"
				id="l-incorrect-color"
				bind:value={lIncorrectColor}
				on:change={() => updateTheme()}
			/>
		</div>

		<div>
			<label for="l-extra-color"> Extra color: </label>
			<input
				type="color"
				id="l-extra-color"
				bind:value={lExtraColor}
				on:change={() => updateTheme()}
			/>
		</div>

		<div>
			<label for="l-incorrect-underline"> Incorrect underline: </label>
			<input
				type="color"
				id="l-incorrect-underline"
				bind:value={lIncorrectUnderline}
				on:change={() => updateTheme()}
			/>
		</div>

		<div>
			<label for="caret-color"> Caret color: </label>
			<input
				type="color"
				id="caret-color"
				bind:value={lCaretColor}
				on:change={() => updateTheme()}
			/>
		</div>
	</div>

	<button
		class="btn"
		on:click={() => {
			updateTheme(true);
			dispatch('close');
		}}
	>
		Update theme
	</button>
</div>

<style>
	.floating-window {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);

		width: 100%;
		max-width: 500px;

		text-align: center;

		z-index: 10000;
	}

	.floating-window > .grid-container {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		grid-gap: 1rem;
		padding: 1rem;
		background-color: var(--bg-color);
		color: var(--fg-color);
	}

	.floating-window > .grid-container > div {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
