type Day = {
    id: string;
    date: number;
    price: number;
    tag: string;
    comment: string;
    save: () => void;
};

type Wasm = Record<string, any>;
const NS = "2025:august"; // todo

const onceMapId = new Set();

export class Listener {
	wasm: Wasm
	ns: string
	popup: Popup | null

	constructor(wasm: Wasm, ns: string) {
		this.wasm = wasm; 
		this.popup = null;
		this.ns = ns;
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
				this.popup = new Popup(this.wasm, this.ns, id, node);
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

function createModel(wasm: Wasm, ns: string, id: string) {
	const model = Boolean(id) 
		? wasm.Day.fetch(id) 
		: wasm.Day.new_with_date(wasm.Store.stats(ns)?.last_date ?? 1);
	if (!Boolean(id)) { onceMapId.add(model.id); }
	return model;
}

class Popup {
	wasm: Wasm
	row: Element
	model: Day
	view: Money | Comment | Tag | Year | null

	constructor(wasm: Wasm, ns: string, id: string, row: Element) {
		this.wasm = wasm;
		this.model = createModel(wasm, ns, id);
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
		action?.startsWith('nav/') ? this.handleNav(action) : this.handleChildren(event);	 
	}

	handleChildren(event: Event) {
		this.view?.action(event);
		if (onceMapId.has(this.model.id)) {
			this.row.setAttribute('__id', this.model.id);
			onceMapId.delete(this.model.id);
			this.wasm.Store.append(NS, this.model);
		}
	}

	handleNav(action: string) {
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
		}
	}

	input(value: number) {
		if (isNaN(value)) return; 
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
				return;
			case 'money/slider-main':
				this.slider(parseInt((event.target as HTMLInputElement).value));
				return;
			case 'money/input':
				this.input(parseFloat((event.target as HTMLInputElement).value));
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
				return;
			case 'tag/input':
				this.input((event.target as HTMLInputElement).value);
		}
	}

	fill(tag: string) { 
		(document.querySelector("#tag-input") as HTMLInputElement).value = tag;
		(document.querySelector("#tag-input") as HTMLInputElement).placeholder = tag;
	}

	slider(value: number) {
		const newTag = this.tags[value % this.tags.length] ?? "no tags yet";
		(document.querySelector("#tag-input") as HTMLInputElement).value = newTag;
		(document.querySelector("#tag-slider-msg") as HTMLInputElement).textContent = newTag[0]?.toUpperCase() ?? "A";
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



