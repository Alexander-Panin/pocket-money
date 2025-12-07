import getWasm from "../common/wasm";
import * as route from "../common/route";
import { getMonthBy } from "../common/utils";

export class View {
	ns: string;

	constructor(ns: string) { this.ns = ns; }

	prerender() {
		document
			.querySelector("#container-list")!
			.appendChild(
				(document.querySelector("#template-list") as HTMLTemplateElement).content
			);

		const {month, year} = route.getParams(this.ns);
		document.querySelector('#list-month-title')!.textContent = `${getMonthBy(month, 'ru')} ${year}`; 
		(document.querySelector('#list-link-title') as HTMLAnchorElement).href = `/index.html?month=${month}&year=${year}`;
	}

	async render() {
		const container = document.querySelector("#container-row")!;
  		const groups = await getWasm().Store.group_by(this.ns);
  		for (const x of groups) {
  			const [category,sum] = [x[0], x[1]]; 
  			const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
  			container.appendChild(this.fill(row.cloneNode(true) as HTMLElement, sum, category));
  		}
	}

	fill(x: HTMLElement, sum: number, category: string) {
  		x.querySelector('#row-sum')!.textContent = String(Math.round(sum * 10) / 10);
  		x.querySelector('#row-category')!.textContent = category || "без категории";
  		return x;
	}
}








