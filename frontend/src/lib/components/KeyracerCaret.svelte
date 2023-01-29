<script lang="ts">
	import type { InputWord } from '$lib/types';

	let caretLeft: number = 9;
	let caretTop: number = 20;

	let caretBlinkTime: number = 5000;
	let caretBlinking: boolean = true;
	let caretBlinkTimeout: any;

	export function processCaret(
		words: InputWord[],
		currentWordIndex: number,
		currentCharIndex: number
	) {
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
			setTimeout(() => processCaret(words, currentWordIndex, currentCharIndex), 0);
		}
	}

	export function setCaretBlinkState(blinking: boolean) {
		caretBlinking = blinking;
	}
</script>

<div class="caret {caretBlinking ? 'blink' : ''}" style="left: {caretLeft}px; top: {caretTop}px" />

<style>
	.caret {
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
</style>
