<script lang="ts">
	import type { MenuItem } from '$lib/types';
	import { changeTheme, getCookie, themes } from '$lib/utils';
	import { createEventDispatcher, onMount } from 'svelte';
	import CustomThemeColor from './CustomThemeColor.svelte';
	import Icon from './SettingsIcon.svelte';

	let dispatch = createEventDispatcher();
	let customThemeSelector: boolean = false;

	let searchElement: HTMLElement;
	let searchString: string = '';

	let currentSelectedPath: string[] = [];
	let currentShownMenu: MenuItem[] = [];
	let currentElementId: number = 0;

	let menu: MenuItem[] = [
		{
			name: 'Themes',
			action: () => {},
			sub: []
		}
	];

	onMount(async () => {
		await getThemes();
		navigation(0, 0);
	});

	async function onKeyDown(event: KeyboardEvent) {
		switch (event.key) {
			case 'ArrowUp':
				event.preventDefault();
				navigation(-1);
				break;

			case 'ArrowDown':
			case 'Tab':
				event.preventDefault();
				navigation(1);
				break;

			case 'Enter':
				event.preventDefault();
				await clickAction(currentShownMenu[currentElementId]);
				break;

			case 'Escape':
				event.preventDefault();
				await backPane();
				break;

			default:
				searchElement.focus();
				break;
		}
	}

	async function search() {
		await calculateMenu();
		navigation(0, 0);

		let tmpMenu: MenuItem[] = [];
		for (let item of currentShownMenu) {
			if (item.name.toLowerCase().includes(searchString.toLowerCase())) {
				tmpMenu.push(item);
			}
		}

		currentShownMenu = tmpMenu;
	}

	function navigation(mv: number, force: number = -1) {
		currentElementId += mv;
		if (force > -1) currentElementId = force;

		if (currentElementId >= currentShownMenu.length) {
			currentElementId = 0;
		} else if (currentElementId < 0) {
			currentElementId = currentShownMenu.length - 1;
		}

		// @ts-ignore
		document.getElementById(`elem-${currentElementId}`).focus();
	}

	async function backPane() {
		let prevName = currentSelectedPath.pop();
		if (!prevName) {
			dispatch('close');
			return;
		}

		await calculateMenu();
		navigation(
			0,
			currentShownMenu.findIndex((x) => x.name === prevName)
		);
	}

	async function getThemes() {
		let currentTheme: string = getCookie('theme') || 'amoled';
		let tmpThemes: MenuItem[] = [];

		for (let theme of themes) {
			tmpThemes.push({
				name: theme,
				icon: theme === currentTheme ? 'checkmark' : '',
				action: async () => {
					changeTheme(theme);
					await getThemes();
				}
			});
		}

		tmpThemes.push({
			name: 'custom',
			icon: currentTheme === 'custom' ? 'checkmark' : '',
			action: async () => {
				customThemeSelector = true;
				changeTheme('custom');
				await getThemes();
			}
		});

		await insertMenu(['Themes'], tmpThemes, true);
	}

	async function clickAction(element: MenuItem) {
		if (element.sub) {
			currentSelectedPath.push(element.name);
			await calculateMenu();
		} else {
			element.action();
		}

		searchString = '';
	}

	async function insertMenu(
		menuPath: string[],
		menuToInsert: MenuItem[],
		replace: boolean = false
	) {
		let tmpMenu = menu;

		while (true) {
			let relativePath = menuPath.shift();
			let tmpIndex = tmpMenu.findIndex((x) => x.name === relativePath);

			if (tmpIndex >= 0) {
				if (menuPath.length === 0) {
					tmpMenu[tmpIndex].sub?.push(...menuToInsert);
					if (replace) tmpMenu[tmpIndex].sub = menuToInsert;

					await calculateMenu();

					return;
				}

				// @ts-ignore
				if (tmpMenu[tmpIndex].sub) tmpMenu = tmpMenu[tmpIndex].sub;
			} else {
				console.error('Wrong path');
				return;
			}
		}
	}

	async function calculateMenu() {
		currentShownMenu = menu;

		for (let i = 0; i < currentSelectedPath.length; i++) {
			const element = currentSelectedPath[i];

			for (let j = 0; j < currentShownMenu.length; j++) {
				const menuElement = currentShownMenu[j];

				if (menuElement.name === element) {
					currentShownMenu = menuElement.sub || [];
					break;
				}
			}
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

{#if customThemeSelector}
	<CustomThemeColor />
{/if}

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="back-blur" on:click={() => dispatch('close')} />

<div class="window">
	<div class="nav">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="2"
			stroke="currentColor"
			style="width: 22px; height: 22px;"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
			/>
		</svg>

		<input
			type="text"
			placeholder="Search"
			bind:value={searchString}
			bind:this={searchElement}
			on:input={async () => await search()}
		/>
	</div>

	{#each currentShownMenu as element, i}
		<button class="element" on:click={async () => await clickAction(element)} id="elem-{i}">
			<Icon type={element.icon} size={18} />
			<span> {element.name} </span>
		</button>
	{/each}
</div>

<style>
	.back-blur {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: 999;
		background-color: rgba(0, 0, 0, 0.5);
	}

	.window {
		position: absolute;

		width: calc(100% - 2em);
		max-width: 600px;

		top: 4em;
		left: 50%;
		transform: translate(-50%, 0);

		z-index: 1000;

		border-radius: 0.5em;
		background-color: var(--bg-color);

		overflow-y: scroll;
	}

	.window::-webkit-scrollbar {
		width: 5px;
	}
	.window::-webkit-scrollbar-track {
		background: transparent;
	}
	.window::-webkit-scrollbar-thumb {
		background: #888;
		border-radius: 10px;
	}
	.window::-webkit-scrollbar-thumb:hover {
		background: #555;
	}

	.nav {
		display: flex;
		align-items: center;

		padding-left: 0.5em;
		padding-right: 0.5em;
	}

	.nav > input {
		height: 22px;
		padding: 16px;

		font-size: 1rem;
		width: 100%;

		margin: 0;
		outline: none;

		border: none;
		background-color: var(--bg-color);
		color: var(--fg-color);
	}

	.element {
		width: 100%;
		height: 28px;

		border: none;
		background-color: var(--bg-color);
		color: var(--fg-color);

		display: flex;
		align-items: center;

		padding-left: 0.5em;

		transition: all 0.2s ease-in-out;
	}
	.element:hover,
	.element:focus {
		background-color: var(--fg-color);
		color: var(--bg-color);

		cursor: pointer;
		outline: none;
	}

	.element > span {
		margin-left: 0.5em;
	}
</style>
