import { Popup } from "./popup";
import getWasm from "../common/wasm";

export class Listener {
	ns: string
	popup: Popup | null

	constructor(ns: string) {
		this.ns = ns;
		this.popup = null;
	}

	handler = (event: Event) => {
		const node = getWasm().target(event.target as Element);
		if (!node?.attributes) return;
		const action = node.attributes.getNamedItem('__action')?.value;
		const id = node.attributes.getNamedItem('__id')?.value ?? "";
		if (action != "ns/skip") event.preventDefault();
		switch (action) {
			case 'row':
				this.focus(node as HTMLElement);
				this.popup?.destroy();
				this.popup = new Popup(id, node, this.ns);
				return;
			case 'list/on-top':
				this.onTop();
				/* pass down */
			default:
				this.popup?.destroy();
		}
	}

	onTop() {
		document
			.querySelector('#list-title')!
			.scrollIntoView({ behavior: "smooth", block: "end" }); 	
	}

    focus(row: HTMLElement) { 
    	row.focus(); 
    	row.scrollIntoView({ 
    		behavior: "smooth", 
    		block: "center", 
    		inline: "center",
    	}); 
    }
}

