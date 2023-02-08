import { settings } from "$lib/stores";
import { setCookie } from "$lib/utils";

export const themes: Map<string, string> = new Map([
    ['amoled', 'black,white,white,gray,red,darkred,red,green'],
    ['dark', '#111111,white,white,gray,red,darkred,red,lime'],
    ['arch', '#0c0d11,#7ebab5,white,gray,red,darkred,red,#7ebab5'],
    ['light', '#f0f0f0,black,black,gray,red,darkred,red,green'],
    ['matcha', '#a4b07e,#6d4930,#6d4930,#825e459e,#905963,#6a363f,#6a363f,#e0e0af9e']
]);

export type MenuItem = {
    name: string;
    icon?: string;
    action: () => void;

    sub?: MenuItem[];
};

export function changeTheme(selectedTheme: string) {
    let theme = selectedTheme.split(',');
    if (theme.length != 8) return;

    setCookie('theme', selectedTheme, 365);

    document.documentElement.style.setProperty("--bg-color", theme[0]);
    document.documentElement.style.setProperty("--fg-color", theme[1]);
    document.documentElement.style.setProperty("--l-correct-color", theme[2]);
    document.documentElement.style.setProperty("--l-ns-color", theme[3]);
    document.documentElement.style.setProperty("--l-incorrect-color", theme[4]);
    document.documentElement.style.setProperty("--l-extra-color", theme[5]);
    document.documentElement.style.setProperty("--w-incorrect-underline", theme[6]);
    document.documentElement.style.setProperty("--caret-color", theme[7]);
}


export type Settings = {
    mode: number;
    customThemes: Map<string, string>;
};

export function loadSettings() {
    let tmpSettings: Settings = {
        mode: 0,
        customThemes: new Map()
    };

    let settingsStr = localStorage.getItem('settings');
    if (settingsStr) {
        tmpSettings = JSON.parse(settingsStr);
    }

    settings.set(tmpSettings);
}

export function saveSettings() {
    settings.subscribe(value => {
        localStorage.setItem('settings', JSON.stringify(value));
    });
}
