<script lang="ts">
	import { onMount } from 'svelte';

	enum CharState {
		NotStarted,
		Extra,
		Correct,
		Incorrect
	}

	type InputChar = {
		val: string;
		elem: HTMLElement | null;
		state: CharState;
	};

	type InputWord = {
		characters: InputChar[];
		finished: boolean;
	};

	let caretElement: Element;
	let caretLeft: number = 9;
	let caretTop: number = 20;

	let caretBlinkTime: number = 5000;
	let caretBlinking: boolean = true;
	let caretBlinkTimeout: any;

	let words: InputWord[];
	let currentWordIndex: number = 0;
	let currentCharIndex: number = 0;

	words = stringToWords('This is a test string to write');
	onMount(() => {
		processCaret();
        caretBlinking = true;
	});

	function onKeyDown(event: KeyboardEvent) {
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
			processBackspace();
		}

		processCaret();
	}

	function processCaret() {
		caretBlinking = false;

		clearTimeout(caretBlinkTimeout);
		caretBlinkTimeout = setTimeout(() => {
			caretBlinking = true;
		}, caretBlinkTime);

		let charIndex =
			currentCharIndex < words[currentWordIndex].characters.length
				? currentCharIndex
				: words[currentWordIndex].characters.length - 1;

		const char = words[currentWordIndex].characters[charIndex];
		if (char.elem) {
			const rect = char.elem.getBoundingClientRect();
			caretLeft = rect.left + (currentCharIndex === charIndex ? 0 : rect.width);
			caretTop = rect.top;
		} else {
			setTimeout(() => processCaret(), 0);
		}
	}

	function processBackspace() {
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

	// TODO: rewrite this
	const allowedKeys =
		'abcdefghijklmnopqrstuwxyzABCDEFGHIJKLMNOPQRSTUWXYZ0123456789!?.,;\'"\\][}{<>_+-=()&*^&%^$%#$!@~`';
	function checkKeyAllowed(event: KeyboardEvent) {
		return allowedKeys.includes(event.key);
	}
</script>

<svelte:body on:keydown={onKeyDown} />
<h2>{currentWordIndex} {currentCharIndex} {words.filter((x) => !x.finished).length}</h2>

<div style="font-size: 2em; margin-left: 2px;">
	<div
		bind:this={caretElement}
		class="bar {caretBlinking ? 'blink' : ''}"
		style="left: {caretLeft}px; top: {caretTop}px"
	/>

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

<pre>
{JSON.stringify(words, null, 2)}
</pre>

<style>
	.bar {
		width: 2px;
		height: 43px;
		background-color: lime;
		position: absolute;
		transition: all 0.1s;
	}

	.blink {
		animation: blink 1s infinite;
	}
	@keyframes blink {
		0% {
			opacity: 0.25;
		}
		50% {
			opacity: 1;
		}
		100% {
			opacity: 0.25;
		}
	}

	word {
		font-family: 'Roboto Mono', monospace;
	}

	letter {
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
