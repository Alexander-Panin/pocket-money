import getWasm from "../../common/wasm";
import worker from "../../common/worker";

export default class Year {
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

	async input(date: number) {
		if (isNaN(date)) return; 
		await worker("save_date", {id: this.model.id, value: String(date)});
		await getWasm().save_date_fast(this.model.id, String(date));
		this.model.date = date;
	}

	async slider(date: number) {
		(document.querySelector("#year-input") as HTMLInputElement).value = String(date);
		await worker("save_date", {id: this.model.id, value: String(date)});
		await getWasm().save_date_fast(this.model.id, String(date));
		this.model.date = date;
	}

	fill(value: number) {
		const tmp = this.ns.split(':');
		const msg = `/ ${tmp[1]} / ${tmp[0]}`; 
		(document.querySelector("#year-msg") as HTMLElement).textContent = msg; 
		(document.querySelector("#year-input") as HTMLInputElement).value = String(value); 
		(document.querySelector("#year-slider") as HTMLInputElement).value = String(value); 
	}
}