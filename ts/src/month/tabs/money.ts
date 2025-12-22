import getWasm from "../../common/wasm";
import worker from "../../common/worker";
import * as utils from "../utils";

export default class Money {
	row: Element
	model: Day

	constructor(model: Day, row: Element) {
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
		const koef = Math.max(value,0);
		const [min, max] = [koef*5, (koef+2)*5];
		document.querySelector('#money-slider-scale-msg')!.textContent = `${min}–${max}`;
		const slider = (document.querySelector('#money-slider-main') as HTMLInputElement);
		slider.min = String(min*10);
		slider.max = String(max*10);
	}

	async slider(value: number) {
		(document.querySelector("#money-input") as HTMLInputElement).value = String(value / 10);
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = utils.euro(value / 10); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = utils.cent(value / 10);
	    await worker("save_price", {id: this.model.id, value: String(value / 10)});
	    await getWasm().save_price_fast(this.model.id, String(value / 10)); 
	    this.model.price = value / 10;
	}

	async input(value: number) {
		if (isNaN(value)) return;
	    (this.row.querySelector('#row-money-euro') as HTMLElement).textContent = utils.euro(value); 
	    (this.row.querySelector('#row-money-cent') as HTMLElement).textContent = utils.cent(value);
	    await worker("save_price", {id: this.model.id, value: String(value)});
	    await getWasm().save_price_fast(this.model.id, String(value)); 
	    this.model.price = value;
	}
}