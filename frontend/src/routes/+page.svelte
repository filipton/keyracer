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
		errored: number[];
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

				if (
					currentBuffer[currentWordIndex].buffer.length == 0 &&
					currentBuffer[currentWordIndex - 1].state !== WordState.Correct
				) {
					currentBuffer[currentWordIndex].state = getBufferState(currentBuffer[currentWordIndex]);

					currentWordIndex--;
				}
			} else if (event.key === ' ') {
				if (currentWord.buffer !== currentWord.word) {
					currentBuffer[currentWordIndex].state = WordState.Incorrect;
				}

				currentWordIndex++;
			} else {
				currentBuffer[currentWordIndex].buffer += event.key;
			}

			currentBuffer[currentWordIndex].state = getBufferState(currentBuffer[currentWordIndex]);
		}
	}

	function getBufferState(buffered: BufferedWord): WordState {
		if (buffered.buffer.length == 0) return WordState.NotStarted;
		if (buffered.buffer.length > buffered.word.length) return WordState.Incorrect;

		let incorrect: boolean = false;
		for (let i = 0; i < buffered.buffer.length; i++) {
			if (buffered.word[i] !== buffered.buffer[i]) {
				incorrect = true;
				buffered.errored.push(i);
			}
		}

		if (incorrect) return WordState.Incorrect;
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
				state: WordState.NotStarted,
				errored: []
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

<h1>Write this text: {currentBuffer.map((x) => x.word).join(' ')}</h1>

<div style="font-size: xx-large;">
	{#each currentBuffer as buffered}
		<span style={getWordColor(buffered)}>{buffered.buffer} </span>
	{/each}
</div>

<div style="font-size: 1.5rem;">
	{#each currentBuffer as buffered}
		<span style="margin: 0.25em; {getWordColor(buffered)}">{buffered.word}</span>
	{/each}
</div>

<div style="font-size: xx-large;">
	{#each currentBuffer as buffered}
		<word>
			{#each buffered.buffer as character, index}
				<letter style={buffered.errored.includes(index) ? 'color: red' : 'color: black'}
					>{character}</letter
				>
			{/each}
		</word>
	{/each}
</div>

<style>
</style>
