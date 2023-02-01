<script lang="ts">
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
			width: 250
		});

		//@ts-ignore
		google.accounts.id.prompt(); // also display the One Tap dialog
	});

	async function handleCredentialResponse(response: any) {
		const { credential } = response;

		if (credential) {
			await fetch('http://localhost:8080/auth', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(credential)
			})
				.then((res) => res.text())
				.then((x) => console.log(x));
		}
	}

	function toggle() {
		showLoginScreen = !showLoginScreen;
	}
</script>

<div class="container">
	<button on:click={() => toggle()} class="btn"> Login </button>
</div>

<div class="googleSignIn" style={showLoginScreen ? '' : 'display: none'}>
	<div bind:this={googleButton} />
</div>

<style>
	.googleSignIn {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);

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
