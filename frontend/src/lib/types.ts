export const apiUrl = "http://127.0.0.1:8080/api";

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

export type User = {
    id: number;
    name: string;
    email: string;
    created_at: bigint;
};

export type NrResult = {
    id: number;
    user_id: number;
    time: number;
    wpm: number;
    acc: number;
    max_ks: number;
    created_at: bigint;
};


export type RankedQuote = {
    id: number;
    start_at: bigint;
    quote: string;
};

export type RankedResponse = {
    time: number;
    quote_id: number;
    chars_written: number;
    chars_correct: number;
    chars_in_correct_words: number;
    history: string;
};

export type RankingHistoryEntry = {
    id: number;
    quote: string;
    start_at: bigint;
};

export type RankingEntry = {
    id: number;
    name: string;
    time: number;
    wpm: number;
    acc: number;
    submitted_at: bigint;
};
