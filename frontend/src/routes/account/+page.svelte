<script lang="ts">
	import { page } from '$app/stores';
	import { apiUrl, type User } from '$lib/types';
	import { onMount } from 'svelte';

	let user: User = $page.data.user;
	let googleButton: HTMLElement;

	onMount(() => {
		if (user) return;

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
				.then((res) => res.json())
				.then((x) => {
					setCookie('token', x, 365);
					location.reload();
				});
		}
	}

	function setCookie(cname: string, cvalue: string, exdays: number) {
		const d = new Date();
		d.setTime(d.getTime() + exdays * 24 * 60 * 60 * 1000);
		let expires = 'expires=' + d.toUTCString();
		document.cookie = cname + '=' + cvalue + ';' + expires + ';path=/';
	}
</script>

<div class="container">
	<div class="googleSignIn" style={user ? 'display: none;' : ''}>
		<div bind:this={googleButton} />
	</div>
</div>

<style>
	.googleSignIn {
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
