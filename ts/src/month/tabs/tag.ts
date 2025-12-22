import getWasm from "../../common/wasm";
import worker from "../../common/worker";
import * as utils from "../utils";

function dedup(xs: string[]) {
	const init = ["продукты", "ресторан", "амазон"];
	const newXs = xs.map(x => x.trim().toLowerCase());
	const tmp = Array.from(new Set(init.concat(newXs)));
	tmp.sort();
	return tmp;
}

export default class Tag {
	row: Element
	model: Day
	tags: string[]

	constructor(model: Day, row: Element, ns: string) {
		this.model = model;
		this.row = row;
		this.tags = [];
		getWasm().Store.tags(ns).then((xs: string[]) => { 
			this.tags = dedup(xs); 
			this.fill(this.model.tag);
		});
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

	async slider(value: number) {
		const newTag = this.tags[value % this.tags.length] ?? "no tags yet";
		(document.querySelector("#tag-input") as HTMLInputElement).value = newTag;
		(document.querySelector("#tag-slider-msg") as HTMLInputElement).textContent = newTag[0]?.toUpperCase() ?? "A";
	    (this.row.querySelector('#row-tag') as HTMLElement).textContent = newTag; 
	    await worker("save_tag", {id: this.model.id, value: newTag});
	    await getWasm().save_tag_fast(this.model.id, newTag);
	    this.model.tag = newTag;
	}

	async input(tag: string) {
		const newTag = tag.trim().toLowerCase();
	    (this.row.querySelector('#row-tag') as HTMLElement).textContent = newTag;
	    await worker("save_tag", {id: this.model.id, value: newTag});
	    await getWasm().save_tag_fast(this.model.id, newTag);
	    this.model.tag = newTag;
	}
}
