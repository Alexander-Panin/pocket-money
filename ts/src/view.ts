type Wasm = Record<string, any>;

type Day = {
    id: string;
    date: number;
    price: number;
    tag: string;
    comment: string;
};

export class View {
	wasm: Wasm
	ns: string;

	constructor(wasm: Wasm, ns: string) {
		this.wasm = wasm;
		this.ns = ns;
	}

	render() {
		document
			.querySelector("#container-list")!
			.appendChild(
				(document.querySelector("#template-list") as HTMLTemplateElement).content
			);
		this.list();
		this.popup();
	}

	popup() {
		document
			.querySelector("#container-popup")!
			.appendChild(
				(document.querySelector("#template-popup") as HTMLTemplateElement).content
			);
		document
			.querySelector('#container-popup-nav')!
			.appendChild(
				(document.querySelector("#template-nav") as HTMLTemplateElement).content
			);
	}

	list() {
		const container = document.querySelector("#container-row")!;
		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		const date = (document.querySelector("#template-date-row") as HTMLTemplateElement).content;
		this.wasm.Store.select(this.ns).forEach((x: [boolean, Day]) => {
			if (x[0]) { container.appendChild(this.fillDate(date.cloneNode(true) as HTMLElement, x[1])); } 
			container.appendChild(this.fill(row.cloneNode(true) as HTMLElement, x[1]));
		});
	}

	fillDate(x: HTMLElement, d: Day): HTMLElement {
		x.querySelector("#date-row")!.textContent = `${d.date}`;
		return x;
	}

	fill(x: HTMLElement, d: Day): HTMLElement {
		x.querySelector("#row-id")!.setAttribute('__id', d.id);
		x.querySelector("#row-money-euro")!.textContent = this.wasm.euro!(d.price);
		x.querySelector("#row-money-cent")!.textContent = this.wasm.cent!(d.price);
		x.querySelector("#row-tag")!.textContent = d.tag;
		x.querySelector("#row-comment")!.textContent = d.comment;
		return x;
	}
}

