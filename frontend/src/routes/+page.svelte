<script lang="ts">
	enum WordState {
		NotStarted,
		PartiallyCorrect,
		Correct,
		Incorrect
	}

	type BufferedWord = {
		word: string;
		buffer: string;
		state: WordState;
	};

	let currentBuffer: BufferedWord[];
	let currentWordIndex: number = 0;
	currentBuffer = stringToWords('this is a test string to write');

	function onKeyDown(event: KeyboardEvent) {
		console.log(event);

		if (checkKeyAllowed(event)) {
			event.preventDefault();
			let currentWord = currentBuffer[currentWordIndex];

			if (event.key === 'Backspace') {
				currentBuffer[currentWordIndex].buffer = currentWord.buffer.slice(0, -1);
			} else if (event.key === ' ') {
				if (currentWord.buffer !== currentWord.word) {
					currentBuffer[currentWordIndex].state = WordState.Incorrect;
				}

				currentWordIndex++;
			} else {
				currentBuffer[currentWordIndex].buffer += event.key;
			}

			currentBuffer[currentWordIndex].state = getBufferState(currentWord);
		}
	}

	function getBufferState(buffered: BufferedWord): WordState {
		for (let i = 0; i < buffered.buffer.length; i++) {
			if (i >= buffered.word.length) return WordState.Incorrect;
			if (buffered.word[i] !== buffered.buffer[i]) return WordState.Incorrect;
		}

		return buffered.buffer.length == buffered.word.length
			? WordState.Correct
			: WordState.PartiallyCorrect;
	}

	function getWordColor(buffered: BufferedWord): string {
		if (buffered.state === WordState.Incorrect) return 'color: red';
		if (buffered.state === WordState.NotStarted) return 'color: gray';

		return 'color: black';
	}

	function stringToWords(text: string): BufferedWord[] {
		return text.split(' ').map((x) => {
			return {
				word: x,
				buffer: '',
				state: WordState.NotStarted
			} as BufferedWord;
		});
	}

	// TODO: rewrite this
	const allowedKeys =
		'abcdefghijklmnopqrstuwxyzABCDEFGHIJKLMNOPQRSTUWXYZ0123456789!?.,;\'"\\][}{<>_+-=()&*^&%^$%#$!@~` ';
	function checkKeyAllowed(event: KeyboardEvent) {
		return allowedKeys.includes(event.key) || event.key === 'Backspace';
	}
</script>

<svelte:body on:keydown={onKeyDown} />

<h1>Write this text: {JSON.stringify(currentBuffer)}</h1>

<div style="font-size: xx-large;">
	{#each currentBuffer as buffered}
		<span style={getWordColor(buffered)}>{buffered.word} </span>
	{/each}
</div>
