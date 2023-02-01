<script lang="ts">
	import { onMount } from 'svelte';

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
            await fetch("http://localhost:8080/auth", 
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(credential)
                }
            )
            .then(res => res.text())
            .then(x => console.log(x));
        }
	}
</script>

<!---<a class="btn">Test</a>--->
<div class="googleSignIn">
	<div id="buttonDiv" bind:this={googleButton} />
	<div class="buttonMask" />
</div>

<style>
	.googleSignIn {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
	}

	.buttonMask {
		width: 250px;
		height: 44px;
		background-color: var(--bg-color);

		position: absolute;
		top: calc(59px + 40px);
		z-index: 1000;
	}
</style>
