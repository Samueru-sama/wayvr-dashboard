import { render } from "preact";
import { Dashboard } from "./dashboard";
import { Globals } from "./globals";

if (!import.meta.env.DEV) {
	document.oncontextmenu = (event) => {
		event.preventDefault()
	}
}


var globals: Globals | null = null;

function Main({ }: {}) {
	if (!globals) {
		globals = new Globals();
	}

	return <Dashboard globals={globals} />
}

render(<Main />, document.getElementById("root")!);

/*
window.addEventListener("erorr", (message) => {
	if (globals) {
		globals.setGlobalError(JSON.stringify(message));
	}
})

window.addEventListener("unhandledrejection", (event) => {
	if (globals) {
		globals.setGlobalError(event.reason);
	}
});
*/