<script lang="ts">
	import { CharState, type InputChar, type InputWord } from '$lib/types';
	import { afterUpdate, createEventDispatcher, onMount } from 'svelte';
	import KeyracerCaret from './KeyracerCaret.svelte';

	export let input: string;
	export let debug: boolean = false;

	let caret: KeyracerCaret;

	let words: InputWord[];
	let currentWordIndex: number = 0;
	let currentCharIndex: number = 0;

	let finished: boolean = false;
	let dispatch = createEventDispatcher();

	words = stringToWords(input);
	onMount(() => {
		caret.processCaret(words, currentWordIndex, currentCharIndex);
		caret.setCaretBlinkState(true);
	});
	afterUpdate(() => {
		caret.processCaret(words, currentWordIndex, currentCharIndex);
	});

	function onKeyDown(event: KeyboardEvent) {
		if (finished) return;

		if (checkKeyAllowed(event)) {
			event.preventDefault();
			processAllowedKey(event.key);
		} else if (event.key === ' ') {
			event.preventDefault();

			if (currentWordIndex + 1 < words.length) {
				currentWordIndex++;
				currentCharIndex = 0;
			}
		} else if (event.key === 'Backspace') {
			event.preventDefault();
			processBackspace(event.ctrlKey);
		}

		caret.processCaret(words, currentWordIndex, currentCharIndex);

		if (
			currentWordIndex === words.length - 1 &&
			currentCharIndex === words[currentWordIndex].characters.length
		) {
			finished = true;
			dispatch('finished');
		}
	}

	function stringToWords(text: string): InputWord[] {
		return text.split(' ').map((x) => {
			return {
				characters: x.split('').map((c) => {
					return {
						val: c,
						state: CharState.NotStarted
					} as InputChar;
				}),
				finished: false
			} as InputWord;
		});
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
	function getLastCharIndex(chars: InputChar[]): number {
		for (let [index, c] of chars.entries()) {
			if (c.state === CharState.NotStarted) {
				return index;
			}
		}

		return chars.length;
	}

	function processAllowedKey(key: string) {
		let chars = words[currentWordIndex].characters;

		if (currentCharIndex + 1 > chars.length) {
			chars.push({
				state: CharState.Extra,
				elem: null,
				val: key
			});
		} else {
			words[currentWordIndex].characters[currentCharIndex].state =
				chars[currentCharIndex].val === key ? CharState.Correct : CharState.Incorrect;
		}

		currentCharIndex++;
		words[currentWordIndex].finished = wordFinished(words[currentWordIndex]);
	}

	function wordFinished(word: InputWord): boolean {
		for (let character of word.characters) {
			if (character.state != CharState.Correct) return false;
		}

		return true;
	}

	function getCharColor(ichar: InputChar): string {
		if (ichar.state === CharState.Incorrect) return 'incorrect';
		if (ichar.state === CharState.Extra) return 'extra';
		if (ichar.state === CharState.NotStarted) return 'not-started';

		return 'correct';
	}

	// TODO: rewrite this
	const allowedKeys =
		'abcdefghijklmnopqrstuwxyzABCDEFGHIJKLMNOPQRSTUWXYZ0123456789!?.,;\'"\\][}{<>_+-=()&*^&%^$%#$!@~`';
	function checkKeyAllowed(event: KeyboardEvent) {
		return allowedKeys.includes(event.key);
	}
</script>

<svelte:window
	on:resize={() => caret.processCaret(words, currentWordIndex, currentCharIndex)}
	on:keydown={onKeyDown}
/>

<div class="words-container">
	<KeyracerCaret bind:this={caret} />

	{#each words as word, wi}
		<word>
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
		<pre>{JSON.stringify(words, null, 2)}</pre>
	</div>
{/if}

<style>
	.words-container {
		font-size: 2em;
		font-family: 'Roboto Mono', monospace;
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
