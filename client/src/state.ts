import * as Cookies from 'es-cookie';

class State {
    darkMode = false;

    constructor() {
        let mode = Cookies.get("darkMode");

        if (mode != undefined) {
            this.darkMode = (mode === "true");

            if (this.darkMode) {
                document.body.setAttribute("theme", this.darkMode ? "dark" : "light");
            }
        }
    }

    inverse() {
        this.darkMode = !this.darkMode;

        Cookies.set("darkMode", this.darkMode.toString(), { expires: 7, sameSite: "strict" });

        document.body.setAttribute("theme", this.darkMode ? "dark" : "light");
    }
}

export let state = new State();