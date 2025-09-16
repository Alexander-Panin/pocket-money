type Wasm = Record<string, Function>;

type Day = {
    id: number;
    date: number;
    price: number;
    tag: string;
    comment: string;
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
		this.renderRows();
		this.renderPopup();
	}

	renderPopup() {
		const popup = document.querySelector("#container-popup")!;
		const tPopup = (document.querySelector("#template-popup") as HTMLTemplateElement).content;
		const tNav = (document.querySelector("#template-nav") as HTMLTemplateElement).content;
		const tMoney = (document.querySelector("#template-money") as HTMLTemplateElement).content;
		popup.appendChild(tPopup);
		popup
			.querySelector('#container-nav')!
			.appendChild(tNav.cloneNode(true));
		popup
			.querySelector('#container-main')!
			.appendChild(tMoney.cloneNode(true));
	}

	renderRows() {
		const container = document.querySelector("#rows")!;
		const tRow = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		const tRowDate = (document.querySelector("#template-row-date") as HTMLTemplateElement).content;
		this.wasm.storage_all!().forEach(([isNextDate, day]: [boolean, Day]) => {
			if (isNextDate) container.appendChild(this.renderDate(tRowDate.cloneNode(true) as HTMLElement, day)); 
			container.appendChild(this.renderRow(tRow.cloneNode(true) as HTMLElement, day));
		});
	}

	renderDate(x: HTMLElement, d: Day): HTMLElement {
		x.querySelector("#date")!.textContent = `${d.date}`;
		return x;
	}

	renderRow(x: HTMLElement, d: Day): HTMLElement {
		x.querySelector("div")!.setAttribute('__id', String(d.id));
		x.querySelector("#money")!.textContent = this.wasm.euro!(d.price);
		x.querySelector("#money2")!.textContent = this.wasm.cent!(d.price);
		x.querySelector("#tag")!.textContent = d.tag;
		x.querySelector("#row-comment")!.textContent = d.comment;
		return x;
	}
}

