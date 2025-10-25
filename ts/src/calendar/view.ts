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

	render() {
		const months = ['august', 'september', 'october', 'november', 'december'];
		const container = document.querySelector("#container-row")!;
		months.forEach(month => {
      		const sum = getWasm().Store.sum(`2025:${month}`);
      		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
      		container.appendChild(this.row(row.cloneNode(true) as HTMLElement, sum, month));
		});
	}

	row(x: HTMLElement, sum: number, month: string) {
  		x.querySelector('#row-sum')!.textContent = String(sum);
  		x.querySelector('#row-link')!.textContent = getMonthBy(month, 'ru');
  		(x.querySelector('#row-link') as HTMLAnchorElement)!.href = `/?month=${month}&year=2025`;
  		return x;
	}
}






