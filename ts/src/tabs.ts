const NS = "2025:august"; // todo

export class Year {
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

export class Comment {
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

export class Money {
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


export class Tag {
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