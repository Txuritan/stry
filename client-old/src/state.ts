import m from "mithril";

import * as Cookies from "./cookie";

class State {
    darkMode = false;
    query = "";

    constructor() {
        let mode = Cookies.get("darkMode");

        if (mode != undefined) {
            this.darkMode = (mode === "true");

            if (this.darkMode) {
                document.body.setAttribute("theme", this.darkMode ? "dark" : "light");
            }
        }
    }

    checkQuery() {
        if (this.query.length === 0) {
            this.query = m.route.param("search");
        }
    }

    inverse() {
        this.darkMode = !this.darkMode;

        Cookies.set("darkMode", this.darkMode.toString(), { expires: 7, sameSite: "strict" });

        document.body.setAttribute("theme", this.darkMode ? "dark" : "light");
    }
}

export let state = new State();