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
