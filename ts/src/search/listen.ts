import getWasm from "../common/wasm";
import {DataHash, record} from "./prefix-hash";

export class Listener {

	nodes: Array<Element>
	dataHash: DataHash

	constructor() {
		this.nodes = [];
		this.dataHash = new DataHash(); 
	}

	handler = (event: Event) => {
		const node = getWasm().target(event.target as Element);
		if (!node?.attributes) return;
		const action = node.attributes.getNamedItem('__action')?.value;
		const id = node.attributes.getNamedItem('__id')?.value ?? "";
		if (action === "ns/skip") return;
		event.preventDefault();
		switch (action) {
			case 'list/search':
				this.search((event.target as HTMLInputElement).value ?? '');
				return;
			case 'list/on-top':
				this.onTop();
				/* pass down */
		}
	}

	search(key: string) {
		if (key.length < 3) return;
		this.nodes.forEach(x => x.remove());
		this.nodes = []; 
		const result = this.dataHash.prefix.find(key);
		this.render(result, key);
	}

	render(xs: number[], key: string) {
		const container = document.querySelector("#container-row")!;
		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		xs.slice(0, 10).forEach(i => {
			const text = record(this.dataHash.index, i);
			container.appendChild(this.fill(row.cloneNode(true) as HTMLElement, text));
			this.nodes.push( container.lastElementChild! );
		});
	}

	fill(x: HTMLElement, text: string) {
  		x.querySelector('#row-result')!.textContent = text;
  		return x;
	}

	onTop() {
		document
			.querySelector('#list-title')!
			.scrollIntoView({ behavior: "smooth", block: "end" }); 	
	}
}

