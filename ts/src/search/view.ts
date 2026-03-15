import getWasm from "../common/wasm";

export class View {
	constructor() { }

	prerender() {
		document
			.querySelector("#container-list")!
			.appendChild(
				(document.querySelector("#template-list") as HTMLTemplateElement).content
			);
	}

	async render() {
		const date = await getWasm().Index.read('prefixhash:', 'date');
		document.querySelector('#list-updated-date')!.textContent = date;
	}
}








