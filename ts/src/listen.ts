type Day = {
    date: number;
    price: number;
    tag: string,
    comment: string,
};

type Wasm = Record<string, Function>;

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
			case 'row':
				this.focus(node as HTMLElement);
				this.popup?.destroy();
				this.popup = new Popup(this.wasm, id, node);
				return;
			case 'nav/close':
				this.popup?.destroy();
				return;
			default:
				this.popup?.destroy();
		}
	}

    focus(row: HTMLElement) { 
    	row.focus(); 
    	row.scrollIntoView({ behavior: "smooth", block: "center" }); 
    }
}

class Popup {
	wasm: Wasm
	row: Element
	model: Day

	constructor(wasm: Wasm, id: number, row: Element) {
		this.wasm = wasm;
		this.model = wasm.storage_by!(id);
		this.row = row;
		this.link();
		this.tab('money');
		this.fillMoney();
	}

	destroy() {
		this.hide();
		this.unlink();
	}

	unlink() {
		const x = document.querySelector("#container-popup");
		x?.removeEventListener('click', this.handler);
		x?.removeEventListener('input', this.handler);
	}

	link() {
		const x = document.querySelector("#container-popup");
		x?.addEventListener('click', this.handler);
		x?.addEventListener('input', this.handler);
	}

	fillMoney() { 
		const price = this.wasm.money!(this.model.price);
		const input = (document.querySelector("#money-input") as HTMLInputElement);
		input.value = price; 
		input.placeholder = price;
	}

	fillComment() { 
		(document.querySelector("#comment") as HTMLInputElement).value = this.model.comment; 
	}

	hide() { (document.querySelector("#container-popup") as HTMLElement).hidden = true; }

	handler = (event: Event) => {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		if (action === 'nav/close') { return; }
		event.stopImmediatePropagation();
		switch (action) {
			case 'money/slider-scale':
				this.moneyScale(parseInt((event.target as HTMLInputElement).value));
				return;
			case 'money/slider-main':
				this.moneySlider(parseInt((event.target as HTMLInputElement).value));
				return;
			case 'money/input':
				this.moneyInput(parseFloat((event.target as HTMLInputElement).value));
				return;
			case 'nav/comment':
				this.tab('comment');
				this.fillComment();
				return;
			case 'nav/money':
				this.tab('money');
				this.fillMoney();
				return;
		}
	}

	tab(page: string) {
		const popup = document.querySelector("#container-popup")!;
		const template = (document.querySelector(`#template-${page}`) as HTMLTemplateElement).content;
		const container = popup.querySelector('#container-popup-main')!;
		container.replaceChildren(template.cloneNode(true));
		(document.querySelector("#container-popup") as HTMLElement).hidden = false; 
	}

	moneyScale(value: number) {
		const [min, max] = [Math.round(25*value/10), Math.round(1.7**value+16)];
		document.querySelector('#money-slider-msg')!.textContent = `${min}–${max}`;
		const slider = (document.querySelector('#moey-slider-main') as HTMLInputElement);
		slider.min = String(min*10);
		slider.max = String(max*10);
	}

	moneySlider(value: number) {
		(document.querySelector("#money-input") as HTMLInputElement).value = String(value / 10);
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = this.wasm.euro!(value / 10); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = this.wasm.cent!(value / 10);
	    this.wasm.storage_save!({...this.model, price: value / 10 }); 
	}

	moneyInput(value: number) {
		if (isNaN(value)) return;
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = this.wasm.euro!(value); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = this.wasm.cent!(value);
	    this.wasm.storage_save!({...this.model, price: value }); 
	}
}



