<script lang="ts">
	import { CharState, type InputWord } from '$lib/types';
	import {
		checkKeyAllowed,
		getCharColor,
		getLastCharIndex,
		stringToWords,
		wordFinished
	} from '$lib/utils';
	import { afterUpdate, createEventDispatcher, onMount } from 'svelte';
	import KeyracerCaret from './KeyracerCaret.svelte';

	export let input: string;
	export let debug: boolean = false;

	let caret: KeyracerCaret;

	let words: InputWord[];
	let currentWordIndex: number = 0;
	let currentCharIndex: number = 0;

	let charsWritten: number = 0;
	let charsCorrect: number = 0;

	let startTime: number = -1;
	let finished: boolean = false;
	let dispatch = createEventDispatcher();

	words = stringToWords(input);
	onMount(() => {
		caret.processCaret(words, currentWordIndex, currentCharIndex);
		caret.setCaretBlinkState(true);
	});
	afterUpdate(() => {
		if (!finished) caret.processCaret(words, currentWordIndex, currentCharIndex);
	});

	function onKeyDown(event: KeyboardEvent) {
		if (finished) return;

		if (checkKeyAllowed(event)) {
			event.preventDefault();
			processAllowedKey(event.key);
		} else if (event.key === ' ') {
			event.preventDefault();
			processSpace();
		} else if (event.key === 'Backspace') {
			event.preventDefault();
			processBackspace(event.ctrlKey);
		}

		caret.processCaret(words, currentWordIndex, currentCharIndex);
		if (currentWordIndex === words.length - 1 && words[currentWordIndex].finished) {
			finishWriting();
		}
	}

	function finishWriting() {
		finished = true;
		dispatch('finished', {
			time: Date.now() - startTime,
			words: words,
			charsWritten: charsWritten,
			charsCorrect: charsCorrect
		});
	}

	function processSpace() {
		charsWritten++;

		if (currentWordIndex + 1 < words.length && currentCharIndex > 0) {
			if (currentCharIndex === words[currentWordIndex].characters.length) {
				charsCorrect++;
			}

			currentWordIndex++;
			currentCharIndex = 0;
		}

		if (currentWordIndex + 1 === words.length && currentCharIndex > 0) {
			finishWriting();
		}
	}

	function processBackspace(ctrlKey: boolean) {
		if (ctrlKey) {
			if (currentCharIndex == 0 && currentWordIndex > 0 && !words[currentWordIndex - 1].finished) {
				currentWordIndex--;
			}

			currentCharIndex = 0;
			words[currentWordIndex].characters = words[currentWordIndex].characters
				.filter((x) => x.state !== CharState.Extra)
				.map((x) => {
					x.state = CharState.NotStarted;
					return x;
				});

			return;
		}

		// remove extra characters
		if (currentCharIndex > 0) {
			if (words[currentWordIndex].characters[currentCharIndex - 1].state == CharState.Extra) {
				words[currentWordIndex].characters = words[currentWordIndex].characters.slice(0, -1);
			} else {
				words[currentWordIndex].characters[currentCharIndex - 1].state = CharState.NotStarted;
			}

			currentCharIndex--;
			words[currentWordIndex].finished = wordFinished(words[currentWordIndex]);

			return;
		}

		if (currentWordIndex > 0 && !words[currentWordIndex - 1].finished) {
			currentCharIndex = getLastCharIndex(words[currentWordIndex - 1].characters);
			currentWordIndex--;
		}
	}

	function processAllowedKey(key: string) {
		let chars = words[currentWordIndex].characters;

		if (currentCharIndex + 1 > chars.length) {
			if (chars.length > 32) return;

			chars.push({
				state: CharState.Extra,
				elem: null,
				val: key
			});
		} else {
			words[currentWordIndex].characters[currentCharIndex].state =
				chars[currentCharIndex].val === key ? CharState.Correct : CharState.Incorrect;

			if (words[currentWordIndex].characters[currentCharIndex].state === CharState.Correct) {
				charsCorrect++;
			}
		}

		currentCharIndex++;
		words[currentWordIndex].finished = wordFinished(words[currentWordIndex]);

		charsWritten++;
		if (startTime === -1) {
			startTime = Date.now();
		}
	}
</script>

<svelte:window
	on:resize={() => caret.processCaret(words, currentWordIndex, currentCharIndex)}
	on:keydown={onKeyDown}
/>

<div class="words-container">
	<KeyracerCaret bind:this={caret} />

	{#each words as word, wi}
		<word class={currentWordIndex > wi && !word.finished ? 'incorrect-word' : ''}>
			{#each word.characters as character, ci}
				<letter
					bind:this={character.elem}
					class="{wi == currentWordIndex && ci == currentCharIndex ? 'current' : ''} {getCharColor(
						character
					)}">{character.val}</letter
				>
			{/each}
		</word>
	{/each}
</div>

{#if debug}
	<div style="width: 100vw;">
		<h1>Debug Info</h1>
		<h2>Word: {currentWordIndex} Char: {currentCharIndex}</h2>
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
		color: black;
	}

	.not-started {
		color: gray;
	}

	.incorrect {
		color: darkred;
	}

	.extra {
		color: red;
	}
</style>
