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
    	row.scrollIntoView({ behavior: "smooth", block: "center" }); 
    }
}

class Popup {
	wasm: Wasm
	row: Element
	model: Day
	view: Money | Comment | null

	constructor(wasm: Wasm, id: number, row: Element) {
		this.wasm = wasm;
		this.model = wasm.storage_by!(id);
		this.row = row;
		this.view = null;
		this.link();
		this.show();
	}


	destroy() {
		this.hide();
		this.unlink();
	}

	link() {
		const x = document.querySelector("#container-popup");
		x?.addEventListener('click', this.handler);
		x?.addEventListener('input', this.handler);
	}

	unlink() {
		const x = document.querySelector("#container-popup");
		x?.removeEventListener('click', this.handler);
		x?.removeEventListener('input', this.handler);
	}

	show() {
		this.tab('money'); // default tab
		this.view = new Money(this.wasm, this.model, this.row);
	}

	hide() { (document.querySelector("#container-popup") as HTMLElement).hidden = true; }

	handler = (event: Event) => {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		if (action !== 'nav/close') { event.stopImmediatePropagation(); }
		if (this.view?.handler(event)) { return; }
		switch (action) {
			case 'nav/comment':
				this.tab('comment');
				this.view = new Comment(this.wasm, this.model.comment);
				return;
			case 'nav/money':
				this.tab('money');
				this.view = new Money(this.wasm, this.model, this.row);
				return;
			case 'nav/tag':
				this.tab('tag');
				this.view = new Tag(this.wasm, this.model, this.row);
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

}

class Comment {
	wasm: Wasm

	constructor(wasm: Wasm, comment: string) {
		this.wasm = wasm;
		this.fill(comment);
	}

	handler(_event: Event) { return false; }

	fill(comment: string) { 
		(document.querySelector("#comment") as HTMLInputElement).value = comment; 
	}

}

class Money {
	wasm: Wasm
	row: Element
	model: Day

	constructor(wasm: Wasm, model: Day, row: Element) {
		this.wasm = wasm;
		this.model = model;
		this.row = row;
		this.fill(wasm.money!(model.price));
	}

	handler(event: Event) {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		switch (action) {
			case 'money/slider-scale':
				this.scale(parseInt((event.target as HTMLInputElement).value));
				return true;
			case 'money/slider-main':
				this.slider(parseInt((event.target as HTMLInputElement).value));
				return true;
			case 'money/input':
				this.input(parseFloat((event.target as HTMLInputElement).value));
				return true;
			default: 
				return false;
		}
	}

	fill(price: string) { 
		(document.querySelector("#money-input") as HTMLInputElement).value = price;
		(document.querySelector("#money-input") as HTMLInputElement).placeholder = price;
	}

	scale(value: number) {
		const [min, max] = [Math.round(25*value/10), Math.round(1.7**value+16)];
		document.querySelector('#money-slider-scale-msg')!.textContent = `${min}–${max}`;
		const slider = (document.querySelector('#money-slider-main') as HTMLInputElement);
		slider.min = String(min*10);
		slider.max = String(max*10);
	}

	slider(value: number) {
		(document.querySelector("#money-input") as HTMLInputElement).value = String(value / 10);
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = this.wasm.euro!(value / 10); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = this.wasm.cent!(value / 10);
	    this.wasm.storage_save!({...this.model, price: value / 10 }); 
	}

	input(value: number) {
		if (isNaN(value)) return;
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = this.wasm.euro!(value); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = this.wasm.cent!(value);
	    this.wasm.storage_save!({...this.model, price: value }); 
	}

}


class Tag {
	wasm: Wasm
	row: Element
	model: Day

	constructor(wasm: Wasm, model: Day, row: Element) {
		this.wasm = wasm;
		this.model = model;
		this.row = row;
		this.fill(model.tag);
	}

	handler(event: Event) {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		switch (action) {
			case 'tag/slider-main':
				this.slider(parseInt((event.target as HTMLInputElement).value));
				return true;
			case 'tag/input':
				this.input((event.target as HTMLInputElement).value);
				return true;
			default: 
				return false;
		}
	}

	fill(price: string) { 
		(document.querySelector("#tag-input") as HTMLInputElement).value = price;
		(document.querySelector("#tag-input") as HTMLInputElement).placeholder = price;
	}

	slider(value: number) {
		const newTag = this.wasm.storage_tag!(value);
		(document.querySelector("#tag-input") as HTMLInputElement).value = newTag;
		(document.querySelector("#tag-slider-msg") as HTMLInputElement).textContent = newTag[0]?.toUpperCase() ?? "";
	    (this.row.querySelector('#row-tag') as HTMLElement).textContent = newTag; 
	    this.wasm.storage_save!({...this.model, tag: newTag }); 
	}

	input(value: string) {
	    (this.row.querySelector('#row-tag') as HTMLElement).textContent = value; 
	    this.wasm.storage_save!({...this.model, tag: value }); 
	}

}



