import { CharState, type InputChar, type InputWord } from "./types";

export function stringToWords(text: string): InputWord[] {
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

export function getLastCharIndex(chars: InputChar[]): number {
    for (let [index, c] of chars.entries()) {
        if (c.state === CharState.NotStarted) {
            return index;
        }
    }

    return chars.length;
}

export function wordFinished(word: InputWord): boolean {
    for (let character of word.characters) {
        if (character.state != CharState.Correct) return false;
    }

    return true;
}

export function getCharColor(ichar: InputChar): string {
    if (ichar.state === CharState.Incorrect) return 'incorrect';
    if (ichar.state === CharState.Extra) return 'extra';
    if (ichar.state === CharState.NotStarted) return 'not-started';

    return 'correct';
}

// TODO: rewrite this
const allowedKeys =
    'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?.,;:\'"\\][}{<>_+-=()&*^&%^$%#$!@~`';
export function checkKeyAllowed(event: KeyboardEvent) {
    return allowedKeys.includes(event.key);
}
