export function render() {
	const theme = window.matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
	window.document.documentElement.setAttribute('data-theme', theme);
}