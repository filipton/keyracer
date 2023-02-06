<script lang="ts">
	export let type: string = '';
	export let size: number = 24;
</script>

<div style="width: {size}px; height: {size}px;">
	{#if type === 'checkmark'}
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="2"
			stroke="currentColor"
		>
			<path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
		</svg>
	{/if}
</div>
