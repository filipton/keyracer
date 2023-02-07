<script lang="ts">
	import { page } from '$app/stores';
	import { settingsMenuActive } from '$lib/stores';

	let userId: string = $page.data.user ? $page.data.user.id : '';

	function toggleMenu() {
		settingsMenuActive.set(!$settingsMenuActive);
	}
</script>

<div class="header">
	<a href="/ranked" title="Ranked">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="2"
			stroke="currentColor"
			style="width: 1.5rem; height: 1.5rem;"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"
			/>
		</svg>
	</a>
	<div style="width: 1.5rem; height: 1.5rem;" />

	<a href="/" class="logo">KEYRACER</a>

	<button on:click={toggleMenu}>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="2"
			stroke="currentColor"
			style="width: 1.5rem; height: 1.5rem;"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
			/>
			<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
		</svg>
	</button>

	<a href="/user/{userId}" title="User">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="2"
			stroke="currentColor"
			style="width: 1.5rem; height: 1.5rem;"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"
			/>
		</svg>
	</a>
</div>

<style>
	.header {
		display: flex;
		justify-content: center;
		align-items: center;
		max-width: 960px;

		padding-left: 1em;
		padding-right: 1em;
		height: 59px;

		margin: 0 auto;
	}

	.header > a,
	button {
		color: var(--fg-color);
		text-decoration: none;
		transition: color 0.5s;

		background: none;
		outline: none;
		border: none;
		cursor: pointer;
	}

	.header > a.logo {
		font-size: 1.2rem;
		font-weight: 600;
		margin-right: auto;
		margin-left: auto;
	}

	.header > a:hover,
	button:hover {
		color: var(--l-ns-color);
	}

	.header > a.selected {
		color: gray;
		border-bottom: 2px solid gray;
	}
</style>
