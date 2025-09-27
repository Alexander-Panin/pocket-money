type Day = {
    id: string;
    date: number;
    price: number;
    tag: string;
    comment: string;
    save: () => void;
};

type Wasm = Record<string, any>;

const NS = "2025:august";

// todo maybe move to wasm
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
		const id = node.attributes.getNamedItem('__id')?.value ?? "hmm...";
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

class Popup {
	wasm: Wasm
	row: Element
	model: Day
	view: Money | Comment | Tag | Year | null

	constructor(wasm: Wasm, id: string, row: Element) {
		this.wasm = wasm;
		this.model = wasm.Day.fetch(id);
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
		if (this.view?.action(event)) { return; }
		switch (action) {
			case 'nav/comment':
				this.tab('comment');
				this.view = new Comment(this.wasm, this.model, this.row);
				return;
			case 'nav/money':
				this.tab('money');
				this.view = new Money(this.wasm, this.model, this.row);
				return;
			case 'nav/tag':
				this.tab('tag');
				this.view = new Tag(this.wasm, this.model, this.row);
				return;
			case 'nav/year':
				this.tab('year');
				this.view = new Year(this.wasm, this.model);
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

class Year {
	wasm: Wasm
	model: Day

	constructor(wasm: Wasm, day: Day) {
		this.wasm = wasm;
		this.model = day;
		this.fill(day.date)
	}

	action(event: Event) {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		switch (action) {
			case 'year/input':
				this.input(parseInt((event.target as HTMLInputElement).value));
				return true;
			default: 
				return false;
		}
	}

	input(value: number) {
		if (isNaN(value) || value < 1) return; // todo think about < 1
		this.model.date = value;
		this.model.save();
	}

	fill(value: number) {
		const msg = `/ 08 / 2025`; // todo later 
		(document.querySelector("#year-msg") as HTMLElement).textContent = msg; 
		(document.querySelector("#year-input") as HTMLInputElement).value = String(value); 
	}
}


class Comment {
	wasm: Wasm
	model: Day
	row: Element

	constructor(wasm: Wasm, model: Day, row: Element) {
		this.wasm = wasm;
		this.model = model;
		this.row = row;
		this.fill(this.model.comment);
	}

	action(event: Event) {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		switch (action) {
			case 'comment':
				this.comment((event.target as HTMLTextAreaElement).value);
				return true;
			default: 
				return false;
		}
	}

	fill(comment: string) { 
		(document.querySelector("#comment") as HTMLInputElement).value = comment;
		(document.querySelector("#comment") as HTMLInputElement).placeholder = comment; 
	}
	comment(comment: string) { 
		(document.querySelector("#comment") as HTMLInputElement).value = comment; 
		(this.row.querySelector('#row-comment') as HTMLElement).textContent = comment; 
		this.model.comment = comment;
		this.model.save();
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

	action(event: Event) {
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
	    this.model.price = value / 10;
	    this.model.save(); 
	}

	input(value: number) {
		if (isNaN(value)) return;
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = this.wasm.euro!(value); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = this.wasm.cent!(value);
	    this.model.price = value;
	    this.model.save(); 
	}

}


class Tag {
	wasm: Wasm
	row: Element
	model: Day
	tags: string[]

	constructor(wasm: Wasm, model: Day, row: Element) {
		this.wasm = wasm;
		this.model = model;
		this.row = row;
		this.tags = wasm.Store.tags(NS);
		this.fill(model.tag);
	}

	action(event: Event) {
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

	fill(tag: string) { 
		(document.querySelector("#tag-input") as HTMLInputElement).value = tag;
		(document.querySelector("#tag-input") as HTMLInputElement).placeholder = tag;
	}

	slider(value: number) {
		const newTag = this.tags[value % this.tags.length] ?? "no tags yet";
		(document.querySelector("#tag-input") as HTMLInputElement).value = newTag;
		(document.querySelector("#tag-slider-msg") as HTMLInputElement).textContent = newTag[0]?.toUpperCase() ?? "";
	    (this.row.querySelector('#row-tag') as HTMLElement).textContent = newTag; 
	    this.model.tag = newTag;
	    this.model.save();
	}

	input(value: string) {
	    (this.row.querySelector('#row-tag') as HTMLElement).textContent = value;
	    this.model.tag = value;
	    this.model.save(); 
	}

}



