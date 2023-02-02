export const apiUrl = "http://localhost:8080";

export enum CharState {
    NotStarted,
    Extra,
    Correct,
    Incorrect
}

export type InputChar = {
    val: string;
    elem: HTMLElement | null;
    state: CharState;
};

export type InputWord = {
    characters: InputChar[];
    finished: boolean;
};

export type KeyracerFinishDetails = {
    time: number;
    words: InputWord[];
    charsWritten: number;
    charsCorrect: number;
    charsInCorrectWords: number;
    history: HistoryEntry[];
};

export type QuoteJson = {
    quote: string;
    author: string;
};

export type KeyracerResponse = {
    time: number;
    chars_written: number;
    chars_correct: number;
    chars_in_correct_words: number;
    history: string;
};

export type HistoryEntry = {
    input: string;
    time: number;
};
