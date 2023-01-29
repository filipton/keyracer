<script lang="ts">
	enum CharState {
		NotStarted,
		Extra,
		Correct,
		Incorrect
	}

	type InputChar = {
		val: string;
		state: CharState;
	};

	type InputWord = {
		characters: InputChar[];
		finished: boolean;
	};

	let words: InputWord[];
	let currentWordIndex: number = 0;
	let currentCharIndex: number = 0;

	words = stringToWords('this is a test string to write');

	function onKeyDown(event: KeyboardEvent) {
		console.log(event);

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
	}

	function processBackspace() {
		if (currentCharIndex > 0) {
			if (words[currentWordIndex].characters[currentCharIndex - 1].state == CharState.Extra) {
				words[currentWordIndex].characters.pop();

				// svelte re-renders elements only when they are "assigned"
				words[currentWordIndex] = words[currentWordIndex];
			} else {
				words[currentWordIndex].characters[currentCharIndex - 1].state = CharState.NotStarted;
			}

			currentCharIndex--;
			return;
		}

		if (currentWordIndex > 0 && !words[currentWordIndex - 1].finished) {
			currentCharIndex = words[currentWordIndex - 1].characters.length;
			currentWordIndex--;
		}
	}

	function processAllowedKey(key: string) {
		let chars = words[currentWordIndex].characters;

		if (currentCharIndex + 1 > chars.length) {
			chars.push({
				state: CharState.Extra,
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

<h1>Write this text: {words.map((x) => x.characters.map((y) => y.val).join('')).join(' ')}</h1>
<h2>{currentWordIndex} {currentCharIndex}</h2>

<div style="font-size: xx-large;">
	{#each words as word}
		<word>
			{#each word.characters as character}
				<letter class={getCharColor(character)}>{character.val}</letter>
			{/each}
		</word>
	{/each}
</div>

<pre>
{JSON.stringify(words, null, 2)}
</pre>

<style>
	.correct {
		color: black;
	}

	.not-started {
		color: gray;
	}

	.incorrect {
		color: red;
	}

	.extra {
		color: darkred;
	}
</style>
