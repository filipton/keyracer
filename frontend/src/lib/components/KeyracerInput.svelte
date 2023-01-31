<script lang="ts">
	import {
		CharState,
		type InputChar,
		type InputWord,
		type KeyracerFinishDetails
	} from '$lib/types';
	import { afterUpdate, createEventDispatcher, onMount } from 'svelte';
	import { KRInput } from './input';
	import KeyracerCaret from './KeyracerCaret.svelte';

	export let input: string;
	export let debug: boolean = false;

	let caret: KeyracerCaret;
	let inputer: KRInput = new KRInput();
	inputer.onFinish = (details: KeyracerFinishDetails) => {
		finished = true;
		dispatch('finished', details);
	};
	inputer.onWordsChanged = (_words: InputWord[]) => {
		words = _words;
	};

	let words: InputWord[] = [];

	let finished: boolean = false;
	let dispatch = createEventDispatcher();

	onMount(() => {
		inputer.init(input);

		caret.processCaret(inputer.words, inputer.currentWordIndex, inputer.currentCharIndex);
		caret.setCaretBlinkState(true);
	});
	afterUpdate(() => {
		if (!finished)
			caret.processCaret(inputer.words, inputer.currentWordIndex, inputer.currentCharIndex);
	});

	function onKeyDown(event: KeyboardEvent) {
		if (finished) return;
		inputer.processKeyEvent(event);

		caret.processCaret(inputer.words, inputer.currentWordIndex, inputer.currentCharIndex);
	}

	function getCharColor(ichar: InputChar): string {
		if (ichar.state === CharState.Incorrect) return 'incorrect';
		if (ichar.state === CharState.Extra) return 'extra';
		if (ichar.state === CharState.NotStarted) return 'not-started';

		return 'correct';
	}
</script>

<svelte:window
	on:resize={() =>
		caret.processCaret(inputer.words, inputer.currentWordIndex, inputer.currentCharIndex)}
	on:keydown={onKeyDown}
/>

<div class="words-container">
	<KeyracerCaret bind:this={caret} />

	{#each words as word, wi}
		<word class={inputer.currentWordIndex > wi && !word.finished ? 'incorrect-word' : ''}>
			{#each word.characters as character, ci}
				<letter
					bind:this={character.elem}
					class="{wi == inputer.currentWordIndex && ci == inputer.currentCharIndex
						? 'current'
						: ''} {getCharColor(character)}">{character.val}</letter
				>
			{/each}
		</word>
	{/each}
</div>

{#if debug}
	<div style="width: 100vw;">
		<h1>Debug Info</h1>
		<pre>{JSON.stringify(
				words.map((x) => {
					return {
						finished: x.finished,
						characters: x.characters.map((c) => {
							return {
								val: c.val,
								state: c.state
							};
						})
					};
				}),
				null,
				2
			)}</pre>
	</div>
{/if}

<style>
	.words-container {
		font-size: 2em;
		max-width: 800px;

		-webkit-user-select: none; /* Safari */
		-ms-user-select: none; /* IE 10 and IE 11 */
		user-select: none; /* Standard syntax */
	}

	.incorrect-word > letter {
		animation: border-bottom 0.5s;
		animation-fill-mode: forwards;
	}
	@keyframes border-bottom {
		0% {
			border-bottom: 3px solid transparent;
		}
		100% {
			border-bottom: 3px solid red;
		}
	}

	.correct {
		color: var(--l-correct-color);
	}

	.not-started {
		color: var(--l-ns-color);
	}

	.incorrect {
		color: red;
	}

	.extra {
		color: darkred;
	}
</style>
