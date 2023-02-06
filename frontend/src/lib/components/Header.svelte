<script lang="ts">
	import { page } from '$app/stores';

	let userId: string = $page.data.user ? $page.data.user.id : '';
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

	<a href="/" class="logo">KEYRACER</a>

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

	.header > a {
		color: var(--fg-color);
		text-decoration: none;
		transition: color 0.5s;
	}

	.header > a.logo {
		font-size: 1.2rem;
		font-weight: 600;
		margin-right: auto;
		margin-left: auto;
	}

	.header > a:hover {
		color: var(--l-ns-color);
	}

	.header > a.selected {
		color: gray;
		border-bottom: 2px solid gray;
	}
</style>
