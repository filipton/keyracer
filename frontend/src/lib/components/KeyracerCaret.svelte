<script lang="ts">
	import type { InputWord } from '$lib/types';

	let caretLeft: number = 9;
	let caretTop: number = 0;

	let caretTopOffset: number = 68;

	let caretBlinkTime: number = 5000;
	let caretEnabled: boolean = false;
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
			caretTop = rect.top - caretTopOffset + scrollY;

			caretEnabled = true;
		} else {
			setTimeout(() => processCaret(words, currentWordIndex, currentCharIndex), 0);
		}
	}

	export function setCaretBlinkState(blinking: boolean) {
		caretBlinking = blinking;
	}
</script>

<div class="caret-container">
	<div
		class="caret {caretBlinking ? 'blink' : ''} {caretEnabled ? 'after-init' : ''}"
		style="left: {caretLeft}px; top: {caretTop}px"
	/>
</div>

<style>
	.caret {
		width: 2px;
		height: 43px;
		position: relative;
	}

	.caret.after-init {
		background-color: lime;
		transition: all 0.1s;
	}

	.caret-container {
		position: absolute;
		top: 68px;
		left: 0;
	}

	.blink.after-init {
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
