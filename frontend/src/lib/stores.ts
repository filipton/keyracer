import { writable, type Writable } from 'svelte/store';
import type { Settings } from './components/SettingsMenu/settingsUtils';
import type { QuoteJson } from './types';

export const settingsMenuActive = writable(false);
export const wordsList: Writable<string[]> = writable([]);
export const quotesList: Writable<QuoteJson[]> = writable([]);

export const settings: Writable<Settings> = writable();
