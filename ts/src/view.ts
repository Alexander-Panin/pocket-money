type Wasm = Record<string, Function>;

type Day = {
    date: number;
    price: number;
    tag: string,
    comment: string,
};

export class View {
	wasm: Wasm

	constructor(wasm: Wasm) {
		this.wasm = wasm;
	}

	render() {
		document
			.querySelector("#list")!
			.appendChild(
				(document.querySelector("#template-list") as HTMLTemplateElement).content
			);
		document
			.querySelector("#popup")!
			.appendChild(
				(document.querySelector("#template-popup") as HTMLTemplateElement).content
			);
		this.renderRows();
	}

	renderRows() {
		const container = document.querySelector("#rows")!;
		const template = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		const templateDate = (document.querySelector("#template-row-date") as HTMLTemplateElement).content;
		console.log(this.wasm.storage_all!());
		this.wasm.storage_all!().forEach(([b, i, day]: [boolean, number, Day]) => {
			if (b) container.appendChild(this.renderDate(templateDate.cloneNode(true) as HTMLElement, day)); 
			container.appendChild(this.renderRow(template.cloneNode(true) as HTMLElement, day, i));
		});
	}

	renderDate(x: HTMLElement, d: Day): HTMLElement {
		x.querySelector("#date")!.textContent = `${d.date}`;
		return x;
	}

	renderRow(x: HTMLElement, d: Day, i: number): HTMLElement {
		console.log(d, i);
		x.querySelector("div")!.setAttribute('__id', String(i));
		x.querySelector("#money")!.textContent = this.wasm.euro!(d.price);
		x.querySelector("#money2")!.textContent = this.wasm.cent!(d.price);
		x.querySelector("#tag")!.textContent = d.tag;
		x.querySelector("#comment")!.textContent = d.comment;
		return x;
	}
}

