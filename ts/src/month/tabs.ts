import * as utils from "./utils";

export class Year {
	model: Day
	ns: string

	constructor(day: Day, ns: string) {
		this.model = day;
		this.ns = ns;
		this.fill(day.date);
	}

	action(event: Event) {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		switch (action) {
			case 'year/input':
				this.input(parseInt((event.target as HTMLInputElement).value));
				return;
			case 'year/slider':
				this.slider(parseInt((event.target as HTMLInputElement).value));
		}
	}

	input(value: number) {
		if (isNaN(value)) return; 
		this.model.date = value;
		this.model.save();
	}

	slider(value: number) {
		(document.querySelector("#year-input") as HTMLInputElement).value = String(value);
	    this.model.date = value;
	    this.model.save(); 
	}

	fill(value: number) {
		const tmp = this.ns.split(':');
		const msg = `/ ${tmp[1]} / ${tmp[0]}`; 
		(document.querySelector("#year-msg") as HTMLElement).textContent = msg; 
		(document.querySelector("#year-input") as HTMLInputElement).value = String(value); 
		(document.querySelector("#year-slider") as HTMLInputElement).value = String(value); 
	}
}

export class Comment {
	model: Day
	row: Element

	constructor(model: Day, row: Element) {
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
		(document.querySelector("#comment") as HTMLInputElement).value = comment ?? "";
		(document.querySelector("#comment") as HTMLInputElement).placeholder = comment ?? ""; 
	}

	comment(comment: string) { 
		(document.querySelector("#comment") as HTMLInputElement).value = comment; 
		(this.row.querySelector('#row-comment') as HTMLElement).textContent = comment; 
		this.model.comment = comment;
		this.model.save();
	}
}

export class Money {
	row: Element
	model: Day

	constructor( model: Day, row: Element) {
		this.model = model;
		this.row = row;
		this.fill(utils.money(model.price));
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
		const koef = value;
		const [min, max] = [Math.round(10*koef), Math.round(10*(koef+1) + 1.7**koef)];
		document.querySelector('#money-slider-scale-msg')!.textContent = `${min}–${max}`;
		const slider = (document.querySelector('#money-slider-main') as HTMLInputElement);
		slider.min = String(min*10);
		slider.max = String(max*10);
	}

	slider(value: number) {
		(document.querySelector("#money-input") as HTMLInputElement).value = String(value / 10);
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = utils.euro(value / 10); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = utils.cent(value / 10);
	    this.model.price = value / 10;
	    this.model.save(); 
	}

	input(value: number) {
		if (isNaN(value)) return;
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = utils.euro(value); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = utils.cent(value);
	    this.model.price = value;
	    this.model.save(); 
	}
}

function dedup(xs: string[]) {
	const init = ["продукты", "ресторан", "амазон"];
	const newXs = xs.map(x => x.trim().toLowerCase());
	const tmp = Array.from(new Set(init.concat(newXs)));
	tmp.sort();
	return tmp;
}

export class Tag {
	row: Element
	model: Day
	tags: string[]

	constructor(model: Day, row: Element, tags: string[]) {
		this.model = model;
		this.row = row;
		this.tags = dedup(tags);
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
		(document.querySelector("#tag-input") as HTMLInputElement).value = tag ?? "";
		(document.querySelector("#tag-input") as HTMLInputElement).placeholder = tag ?? "";
		const slider = (document.querySelector('#tag-slider-main') as HTMLInputElement);
		slider.max = String(Math.min(Number(slider.max), this.tags.length)); 
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
		const newValue = value.trim().toLowerCase();
	    (this.row.querySelector('#row-tag') as HTMLElement).textContent = newValue;
	    this.model.tag = newValue;
	    this.model.save(); 
	}

}
