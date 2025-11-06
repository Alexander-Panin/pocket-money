import getWasm from "../common/wasm";
import { getMonthBy } from "../common/utils";
import * as utils from "./utils";
import * as route from "./route";

export class View {
	ns: string
	constructor(ns: string) { this.ns = ns; }

	prerender() {
		document
			.querySelector("#container-list")!
			.appendChild(
				(document.querySelector("#template-list") as HTMLTemplateElement).content
			);
		const {month, year} = route.getParams(this.ns);
		document.querySelector('#list-title')!.textContent = `${getMonthBy(month, 'ru')} ${year}`; 

		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		const container = document.querySelector("#container-row")!;
		container.appendChild(row.cloneNode(true));
		container.appendChild(row.cloneNode(true));
		container.appendChild(row.cloneNode(true));
	}

	async render() {
		const tmp = await getWasm().Store.select(this.ns) ?? [];
		const days = tmp ?? [];
		this.list(days);
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

	list(days: [boolean, Day][]) {
		const container = document.querySelector("#container-row")!;
		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		const date = (document.querySelector("#template-date-row") as HTMLTemplateElement).content;
		days.forEach((x: [boolean, Day]) => {
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
		x.querySelector("#row-money-euro")!.textContent = utils.euro(d.price);
		x.querySelector("#row-money-cent")!.textContent = utils.cent(d.price);
		x.querySelector("#row-tag")!.textContent = d.tag;
		x.querySelector("#row-comment")!.textContent = d.comment;
		return x;
	}
}

