<script lang="ts">
	import { settings } from '$lib/stores';
	import { getCookie, setCookie } from '$lib/utils';
	import { createEventDispatcher, onMount } from 'svelte';
	import { saveSettings, type Settings } from './settingsUtils';

	let dispatch = createEventDispatcher();

	let customName: string = 'custom';

	let bgColor: string = '#000000';
	let fgColor: string = '#ffffff';
	let lCorrectColor: string = '#ffffff';
	let lNsColor: string = '#808080';
	let lIncorrectColor: string = '#ff0000';
	let lExtraColor: string = '#8B0000';
	let lIncorrectUnderline: string = '#ff0000';
	let lCaretColor: string = '#00ff00';

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

	function onNameChange() {
		if (customName == '') customName = 'custom';

		let customThemes = $settings.customThemes;
		if (customThemes !== null && Object.keys(customThemes).includes(customName)) {
			// @ts-ignore
			let theme = customThemes[customName];
			if (theme !== undefined) {
				let themeArr = theme.split(',');

				bgColor = themeArr[0];
				fgColor = themeArr[1];
				lCorrectColor = themeArr[2];
				lNsColor = themeArr[3];
				lIncorrectColor = themeArr[4];
				lExtraColor = themeArr[5];
				lIncorrectUnderline = themeArr[6];
				lCaretColor = themeArr[7];

				updateTheme();
			}
		}
	}

	function updateTheme(changeCookie: boolean = false) {
		let customTheme = `${bgColor},${fgColor},${lCorrectColor},${lNsColor},${lIncorrectColor},${lExtraColor},${lIncorrectUnderline},${lCaretColor}`;

		document.documentElement.style.setProperty('--bg-color', bgColor);
		document.documentElement.style.setProperty('--fg-color', fgColor);
		document.documentElement.style.setProperty('--l-correct-color', lCorrectColor);
		document.documentElement.style.setProperty('--l-ns-color', lNsColor);
		document.documentElement.style.setProperty('--l-incorrect-color', lIncorrectColor);
		document.documentElement.style.setProperty('--l-extra-color', lExtraColor);
		document.documentElement.style.setProperty('--l-incorrect-underline', lIncorrectUnderline);
		document.documentElement.style.setProperty('--l-caret-color', lCaretColor);

		if (!changeCookie) return;
		setCookie('theme', customTheme, 365);

		let _settings: Settings = $settings;
		if (!$settings.customThemes) {
			_settings.customThemes = new Map();
		}

		// @ts-ignore
		_settings.customThemes[customName] = customTheme;

		settings.set(_settings);
		saveSettings();
	}
</script>

<div class="floating-window">
	<div class="container">
		<div class="pane">
			<label for="bg-color">
				Background color
				<input type="color" id="bg-color" bind:value={bgColor} on:change={() => updateTheme()} />
			</label>

			<label for="fg-color">
				Foreground color
				<input type="color" id="fg-color" bind:value={fgColor} on:change={() => updateTheme()} />
			</label>

			<label for="l-correct-color">
				Correct color
				<input
					type="color"
					id="l-correct-color"
					bind:value={lCorrectColor}
					on:change={() => updateTheme()}
				/>
			</label>

			<label for="l-ns-color">
				Not started color
				<input type="color" id="l-ns-color" bind:value={lNsColor} on:change={() => updateTheme()} />
			</label>
		</div>

		<div class="pane">
			<label for="l-incorrect-color">
				Incorrect color
				<input
					type="color"
					id="l-incorrect-color"
					bind:value={lIncorrectColor}
					on:change={() => updateTheme()}
				/>
			</label>

			<label for="l-extra-color">
				Extra color
				<input
					type="color"
					id="l-extra-color"
					bind:value={lExtraColor}
					on:change={() => updateTheme()}
				/>
			</label>

			<label for="l-incorrect-underline">
				Incorrect underline
				<input
					type="color"
					id="l-incorrect-underline"
					bind:value={lIncorrectUnderline}
					on:change={() => updateTheme()}
				/>
			</label>

			<label for="caret-color">
				Caret color
				<input
					type="color"
					id="caret-color"
					bind:value={lCaretColor}
					on:change={() => updateTheme()}
				/>
			</label>
		</div>
	</div>

	<div class="update-theme">
		<label for="custom-name">Custom name</label>
		<input type="text" id="custom-name" bind:value={customName} on:change={onNameChange} />
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
</div>

<style>
	.floating-window {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);

		width: 100%;
		max-width: 600px;

		text-align: center;
		background-color: var(--bg-color);
		border-radius: 10px;

		z-index: 10000;
	}

	.update-theme {
		display: flex;
		flex-direction: column;
		align-items: center;

		width: 100%;
		margin: 1em auto;
	}
	.update-theme > input {
		border: 3px solid var(--fg-color);
		background-color: var(--bg-color);
		border-radius: 5px;
		color: var(--fg-color);
		padding: 15px 0;
		text-align: center;
		font-size: 1em;
		margin: 4px 2px;
		outline: none;
	}

	.floating-window > .container {
		padding: 1em;

		display: flex;
		flex-direction: row;
		justify-content: space-between;

		color: var(--fg-color);
	}

	.pane {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.pane > label {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
	}

	input {
		margin-left: 10px;

		border: 0;
		padding: 0;
	}
</style>
