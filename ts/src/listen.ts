import { Popup } from "./popup";

export class Listener {
	wasm: Wasm
	popup: Popup | null

	constructor(wasm: Wasm, ns: string) {
		this.wasm = wasm; 
		this.popup = null;
	}

	handler = (event: Event) => {
		const node = this.wasm.target(event.target as Element);
		if (!node?.attributes) return;
		const action = node.attributes.getNamedItem('__action')?.value;
		const id = node.attributes.getNamedItem('__id')?.value ?? "";
		switch (action) {
			case 'row':
				this.focus(node as HTMLElement);
				this.popup?.destroy();
				this.popup = new Popup(this.wasm, id, node);
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

