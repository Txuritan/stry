export let state = {
    darkMode: false,
    inverseMode: () => {
        state.darkMode = !state.darkMode;

        document.dispatchEvent(new CustomEvent("stryStateDarkMode", { detail: state.darkMode }))
    }
};