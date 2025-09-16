type Day = {
    date: number;
    price: number;
    tag: string,
    comment: string,
};

type Wasm = Record<string, Function>;

let __popupHandler = (ev: Event) => {};

function target(node: Element | null): Element | null {
	while (node?.attributes && !node?.attributes?.getNamedItem('__action')?.value) { 
		node = node?.parentNode as Element | null; 
	} 
	return node;
}

export class Listener {
	wasm: Wasm
	popup: Popup | null

	constructor(wasm: Wasm) {
		this.wasm = wasm; 
		this.popup = null;
	}

	handler = (event: Event) => {
		const node = target(event.target as Element);
		if (!node?.attributes) return;
		const action = node.attributes.getNamedItem('__action')?.value;
		const id = parseInt(node.attributes.getNamedItem('__id')?.value ?? "0"); // todo 0
		switch (action) {
			case 'list/row':
				this.focus(node as HTMLElement);
				this.popup?.destroy();
				this.popup = new Popup(this.wasm, id, node);
				return;
			case 'popup/close':
				this.popup?.destroy();
				return;
		}

	}

    focus(row: HTMLElement) { 
    	row.focus(); 
    	row.scrollIntoView({ behavior: "smooth", block: "center" }); 
    }
}

class Popup {
	wasm: Wasm
	node: Element
	id: number

	constructor(wasm: Wasm, id: number, node: Element) {
		this.wasm = wasm;
		this.id = id;
		this.node = node;
		this.link();
		this.show(id);
	}

	destroy() {
		this.hide();
		this.unlink();
	}

	unlink() {
		const x = document.querySelector("#popup");
		x?.removeEventListener('click', this.handler);
		x?.removeEventListener('input', this.handler);
	}

	link() {
		const x = document.querySelector("#popup");
		x?.addEventListener('click', this.handler);
		x?.addEventListener('input', this.handler);
	}

	show(id: number) { 
		const day = this.wasm.storage_by!(id); 
		const value = this.wasm.money!(day.price);
		(document.querySelector("#popup-input") as HTMLInputElement).value = value; 
		(document.querySelector("#popup") as HTMLElement).hidden = false; 
	}

	hide() { (document.querySelector("#popup") as HTMLElement).hidden = true; }

	handler = (event: Event) => {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		switch (action) {
			case 'popup/slider-scale':
				this.scale(parseInt((event.target as HTMLInputElement).value));
				return;
			case 'popup/slider-main':
				this.slider(parseInt((event.target as HTMLInputElement).value));
				return;
			case 'popup/input':
				this.input(parseFloat((event.target as HTMLInputElement).value));
				return;
		}
	}

	scale(value: number) {
		const [min, max] = [Math.round(25*value/10), Math.round(1.7**value+16)];
		document.querySelector('#popup-slider-msg')!.textContent = `${min}–${max}`;
		(document.querySelector('#popup-slider-main') as HTMLInputElement).min = String(min*10);
		(document.querySelector('#popup-slider-main') as HTMLInputElement).max = String(max*10);
	}

	slider(value: number) {
		(document.querySelector("#popup-input") as HTMLInputElement).value = String(value / 10);
	    (this.node.querySelector('#money') as HTMLElement).textContent = this.wasm.euro!(value / 10); 
	    (this.node.querySelector('#money2') as HTMLElement).textContent = this.wasm.cent!(value / 10);
	    this.wasm.storage_save!(this.id, value / 10); 
	}

	input(value: number) {
		if (isNaN(value)) return;
	    (this.node.querySelector('#money') as HTMLElement).textContent = this.wasm.euro!(value); 
	    (this.node.querySelector('#money2') as HTMLElement).textContent = this.wasm.cent!(value);
	    this.wasm.storage_save!(this.id, value); 
	}
}



