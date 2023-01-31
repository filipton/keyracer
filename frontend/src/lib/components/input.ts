import { CharState, type HistoryEntry, type InputChar, type InputWord, type KeyracerFinishDetails } from "$lib/types";

export class KRInput {
    public words: InputWord[] = [];
    private history: HistoryEntry[] = [];

    public currentWordIndex: number = 0;
    public currentCharIndex: number = 0;
    private charsWritten: number = 0;
    private charsCorrect: number = 0;
    private startTime: number = -1;

    public onFinish: (details: KeyracerFinishDetails) => void = () => { };
    public onWordsChanged: (_words: InputWord[]) => void = () => { };

    public init(input: string) {
        this.currentWordIndex = 0;
        this.currentCharIndex = 0;
        this.charsWritten = 0;
        this.charsCorrect = 0;
        this.startTime = -1;
        this.history = [];

        this.words = this.stringToWords(input);
        this.onWordsChanged(this.words);
    }

    public processKeyEvent(event: KeyboardEvent) {
        if (this.checkKeyAllowed(event)) {
            event.preventDefault();
            this.processAllowedKey(event.key);

            this.history.push({
                input: event.key,
                time: Date.now() - this.startTime
            });
        } else if (event.key === ' ') {
            event.preventDefault();
            this.processSpace();

            this.history.push({
                input: ' ',
                time: Date.now() - this.startTime
            });
        } else if (event.key === 'Backspace') {
            event.preventDefault();
            this.processBackspace(event.ctrlKey);

            this.history.push({
                input: `${event.ctrlKey ? '^' : ''}Bp`,
                time: Date.now() - this.startTime
            });
        }

        this.onWordsChanged(this.words);
        if (this.currentWordIndex === this.words.length - 1 && this.words[this.currentWordIndex].finished) {
            this.processFinish();
        }
    }

    private processSpace() {
        this.charsWritten++;

        if (this.currentWordIndex + 1 < this.words.length && this.currentCharIndex > 0) {
            if (this.currentCharIndex === this.words[this.currentWordIndex].characters.length) {
                this.charsCorrect++;
            }

            this.currentWordIndex++;
            this.currentCharIndex = 0;
        }

        if (this.currentWordIndex + 1 === this.words.length && this.currentCharIndex > 0) {
            this.processFinish();
        }
    }

    private processBackspace(ctrlKey: boolean) {
        if (ctrlKey) {
            if (this.currentCharIndex == 0 && this.currentWordIndex > 0 && !this.words[this.currentWordIndex - 1].finished) {
                this.currentWordIndex--;
            }

            this.currentCharIndex = 0;
            this.words[this.currentWordIndex].characters = this.words[this.currentWordIndex].characters
                .filter((x) => x.state !== CharState.Extra)
                .map((x) => {
                    x.state = CharState.NotStarted;
                    return x;
                });

            return;
        }

        // remove extra characters
        if (this.currentCharIndex > 0) {
            if (this.words[this.currentWordIndex].characters[this.currentCharIndex - 1].state == CharState.Extra) {
                this.words[this.currentWordIndex].characters = this.words[this.currentWordIndex].characters.slice(0, -1);
            } else {
                this.words[this.currentWordIndex].characters[this.currentCharIndex - 1].state = CharState.NotStarted;
            }

            this.currentCharIndex--;
            this.words[this.currentWordIndex].finished = this.wordFinished(this.words[this.currentWordIndex]);

            return;
        }

        if (this.currentWordIndex > 0 && !this.words[this.currentWordIndex - 1].finished) {
            this.currentCharIndex = this.getLastCharIndex(this.words[this.currentWordIndex - 1].characters);
            this.currentWordIndex--;
        }
    }

    private processAllowedKey(key: string) {
        let chars = this.words[this.currentWordIndex].characters;

        if (this.currentCharIndex + 1 > chars.length) {
            if (chars.length > 32) return;

            chars.push({
                state: CharState.Extra,
                elem: null,
                val: key
            });
        } else {
            this.words[this.currentWordIndex].characters[this.currentCharIndex].state =
                chars[this.currentCharIndex].val === key ? CharState.Correct : CharState.Incorrect;

            if (this.words[this.currentWordIndex].characters[this.currentCharIndex].state === CharState.Correct) {
                this.charsCorrect++;
            }
        }

        this.currentCharIndex++;
        this.words[this.currentWordIndex].finished = this.wordFinished(this.words[this.currentWordIndex]);

        this.charsWritten++;
        if (this.startTime === -1) {
            this.startTime = Date.now();
        }
    }

    private processFinish() {
        let filtered_words = this.words.filter((x) => x.finished);
        let correct_chars_iw =
            filtered_words.reduce((total, curr) => (total += curr.characters.length), 0) +
            filtered_words.length;

        this.onFinish({
            time: Date.now() - this.startTime,
            words: this.words,
            charsWritten: this.charsWritten,
            charsCorrect: this.charsCorrect,
            charsInCorrectWords: correct_chars_iw,
            history: this.history
        } as KeyracerFinishDetails);
    }


    private stringToWords(text: string): InputWord[] {
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

    private getLastCharIndex(chars: InputChar[]): number {
        for (let [index, c] of chars.entries()) {
            if (c.state === CharState.NotStarted) {
                return index;
            }
        }

        return chars.length;
    }

    private wordFinished(word: InputWord): boolean {
        for (let character of word.characters) {
            if (character.state != CharState.Correct) return false;
        }

        return true;
    }


    // TODO: rewrite this
    allowedKeys =
        'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?.,;:\'"\\][}{<>_+-=()&*^&%^$%#$!@~`';
    private checkKeyAllowed(event: KeyboardEvent): boolean {
        return this.allowedKeys.includes(event.key);
    }
}
