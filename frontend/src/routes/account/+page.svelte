<script lang="ts">
	import { apiUrl } from '$lib/types';
	import { onMount } from 'svelte';

	let showLoginScreen: boolean = false;
	let googleButton: HTMLElement;

	onMount(() => {
		//@ts-ignore
		google.accounts.id.initialize({
			client_id: '1038308532058-lamiirk7j3jko2uc5qhf7clq4e1f1ahp.apps.googleusercontent.com',
			callback: handleCredentialResponse
		});

		//@ts-ignore
		google.accounts.id.renderButton(googleButton, {
			type: 'standard',
			size: 'large',
			theme: 'outline',
			width: 200
		});

		//@ts-ignore
		google.accounts.id.prompt(); // also display the One Tap dialog
	});

	async function handleCredentialResponse(response: any) {
		const { credential } = response;

		if (credential) {
			await fetch(`${apiUrl}/auth`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(credential)
			})
				.then((res) => res.text())
				.then((x) => alert(x));
		}
	}
</script>

<div class="container">
	<div class="googleSignIn">
		<div bind:this={googleButton} />
	</div>
</div>

<style>
	.googleSignIn {
		position: relative;
		top: 0;
		left: 0;

		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
	}

	.container {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
	}
</style>
