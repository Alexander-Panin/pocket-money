import getWasm from "../common/wasm";
import * as route from "../common/route";
import { getMonthBy } from "../common/utils";

export class View {
	ns: string;
	tmpNodes: Array<Element>;

	constructor(ns: string) { 
		this.ns = ns; 
		this.tmpNodes = [];
	}

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
		const prevNs = route.getPrevNamespace(this.ns);
  		const tmp = await getWasm().Store.group_by_months_fast(this.ns, prevNs);
  		this.renderGroups(tmp);
  		const groups = await getWasm().Store.group_by_months(this.ns, prevNs);
		this.tmpNodes.forEach(x => x.remove());
  		this.renderGroups(groups);
		this.tmpNodes = [];
	}

	renderGroups(groups: Array<[string, number, number]>) {
		const container = document.querySelector("#container-row")!;
  		for (const x of groups) {
  			const [category, sum, delta] = [x[0], x[1], x[2]]; 
  			const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
  			const elem = this.fill(row.cloneNode(true) as HTMLElement, sum, delta, category);
  			this.tmpNodes.push(elem);
  			container.appendChild(elem);
  		}
	}

	fill(x: HTMLElement, sum: number, delta: number, category: string): HTMLElement {
		const round = (x: number) => String(Math.round(x * 10) / 10);
  		x.querySelector('#row-sum')!.textContent = round(sum);
  		x.querySelector('#row-delta')!.textContent = round(delta);
  		x.querySelector('#row-category')!.textContent = category || "без категории";
  		return x;
	}
}








