export const themes: Map<string, string> = new Map([
    ['amoled', 'black,white,white,gray,red,darkred,red,green'],
    ['dark', '#111111,white,white,gray,red,darkred,red,lime'],
    ['arch', '#0c0d11,#7ebab5,white,gray,red,darkred,red,#7ebab5'],
    ['light', '#f0f0f0,black,black,gray,red,darkred,red,green'],
    ['matcha', '#a4b07e,#6d4930,#6d4930,#825e459e,#905963,#6a363f,#6a363f,#e0e0af9e']
]);

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


export function setCookie(name: string, value: string, days: number) {
    let expires = '';
    if (days) {
        let date = new Date();
        date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
        expires = '; expires=' + date.toUTCString();
    }
    document.cookie = name + '=' + (value || '') + expires + '; path=/';
}

export function getCookie(name: string) {
    let nameEQ = name + '=';
    let ca = document.cookie.split(';');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) == ' ') c = c.substring(1, c.length);
        if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
    }
    return null;
}
