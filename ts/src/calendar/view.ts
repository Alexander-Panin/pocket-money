import getWasm from "../common/wasm";
import { getMonthBy } from "../common/utils";

export class View {
	constructor() {}

	prerender() {
		document
			.querySelector("#container-list")!
			.appendChild(
				(document.querySelector("#template-list") as HTMLTemplateElement).content
			);
	}

	async render() {
		const months = ['august', 'september', 'october', 'november', 'december'];
		const container = document.querySelector("#container-row")!;
		for (const month of months) {
			// TODO 2025
      		const sum = await getWasm().Store.sum(`2025:${month}`);
      		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
      		container.appendChild(this.fill(row.cloneNode(true) as HTMLElement, sum, month));
		};
	}

	fill(x: HTMLElement, sum: number, month: string) {
  		x.querySelector('#row-sum')!.textContent = String(sum);
  		x.querySelector('#row-link')!.textContent = getMonthBy(month, 'ru');
  		// TODO 2025
  		(x.querySelector('#row-link') as HTMLAnchorElement)!.href = `/?month=${month}&year=2025`;
  		x.querySelector('#row-download')!.setAttribute('__id', `2025:${month}`);
  		x.querySelector('#row-download')!.setAttribute('download', `2025_${month}.csv`);
  		return x;
	}
}








