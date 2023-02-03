<div class="header">
	<div style="width: 1.5rem; height: 1.5rem;" />

	<a href="/" class="logo">KEYRACER</a>

	<a href="/user">
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
		padding: 15px;

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
		color: gray;
	}

	.header > a.selected {
		color: gray;
		border-bottom: 2px solid gray;
	}
</style>
