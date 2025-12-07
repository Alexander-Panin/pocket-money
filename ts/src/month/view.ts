import getWasm from "../common/wasm";
import * as route from "../common/route";
import { getMonthBy } from "../common/utils";
import * as utils from "./utils";

export class View {
	ns: string
	tmpNodes: Array<Element>
	constructor(ns: string) { this.ns = ns; this.tmpNodes = []; }

	prerender() {
		document
			.querySelector("#container-list")!
			.appendChild(
				(document.querySelector("#template-list") as HTMLTemplateElement).content
			);
		const {month, year} = route.getParams(this.ns);
		document.querySelector('#list-title')!.textContent = `${getMonthBy(month, 'ru')} ${year}`; 
		(document.querySelector('#list-stats-link') as HTMLAnchorElement).href = `/stats.html?month=${month}&year=${year}`;

		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		const container = document.querySelector("#container-row")!;
		container.appendChild(row.cloneNode(true));
		container.appendChild(row.cloneNode(true));
		container.appendChild(row.cloneNode(true));
	}

	async render() {
		await Promise.all([
			this.renderFastMemory(),
			this.renderSlowMemory()
		]);
		this.cleanUpFastRender();
	}

	cleanUpFastRender() {
		this.tmpNodes.forEach(x => x.remove()); 
		this.tmpNodes = [];
	}

	async renderSlowMemory() {
		const days = await getWasm().Store.select(this.ns, 1 /* desc */);
		this.list(days);
		this.popup();
		this.repeatRegular(days);
	}

	async renderFastMemory() {
		const days = await getWasm().Store.select_fast(this.ns, 1 /* desc */);
		this.tmpNodes = this.list(days);
	}

	async repeatRegular(days: [boolean, Day][]) {
		if (!Boolean(days.find(x => x[1].date === 0))) {
			const prevNs = route.getPrevNamespace(this.ns);
			const regular = await getWasm().Store.repeat_regular(this.ns, prevNs);
			this.list(getWasm().Store.transform(regular));
		}
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

	list(days: [boolean, Day][]): Array<Element> {
		const container = document.querySelector("#container-row")!;
		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
		const date = (document.querySelector("#template-date-row") as HTMLTemplateElement).content;
		const result: Array<Element> = [];
		days.forEach((x: [boolean, Day]) => {
			const [isNext, day] = [x[0], x[1]];
			if (isNext) {
				const node = date.cloneNode(true) as HTMLElement; 
				container.appendChild(this.fillDate(node, day)); 
				result.push( container.lastElementChild! );
			}
			const node = row.cloneNode(true) as HTMLElement;
			container.appendChild(this.fill(node, day));
			result.push( container.lastElementChild! );
		});
		return result;
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

