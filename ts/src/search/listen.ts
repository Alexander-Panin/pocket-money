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
		this.nodes.forEach(x => x.remove());
		this.nodes = []; 
		if (key.length < 3) return;
		const result = this.dataHash.prefix.find(key);
		this.render(result, key);
	}

	render(xs: number[], key: string) {
		const container = document.querySelector("#container-row")!;
		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		xs.slice(0, 10).forEach(i => {
			const text = record(this.dataHash.index, i);
			container.appendChild(this.fill(row.cloneNode(true) as HTMLElement, text, key));
			this.nodes.push( container.lastElementChild! );
		});
	}

	fill(x: HTMLElement, text: string, key: string) {
		const f = text.indexOf(key);
		const l = f + key.length;
		const [a,b,c] = [text.slice(0,f), text.slice(f,l), text.slice(l)];
		const node = x.querySelector('#row-result-value')!;
		node.before(document.createTextNode(a) );
		node.textContent = b;
		node.after(document.createTextNode(c) );
  		return x;
	}

	onTop() {
		document
			.querySelector('#list-title')!
			.scrollIntoView({ behavior: "smooth", block: "end" }); 	
	}
}

