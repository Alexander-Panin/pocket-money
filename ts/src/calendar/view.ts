import getWasm from "../common/wasm";
import { getMonthBy } from "../common/utils";

const MONTHS_2025 = ['august', 'september', 'october', 'november', 'december'];
const MONTHS_2026 = ['january', 'february', 'march'];

export class View {
	tmpNodes: Array<Element>
	constructor() { this.tmpNodes = []; }

	prerender() {
		document
			.querySelector("#container-list")!
			.appendChild(
				(document.querySelector("#template-list") as HTMLTemplateElement).content
			);
		this.tmpNodes.push(
			...this.list(2025, MONTHS_2025.map((month) => ([month, 0]))),
			...this.list(2026, MONTHS_2026.map((month) => ([month, 0])))
		);
	}

	async money(year: number, months: string[]): Promise<Array<[string, number]>> {
		const result = [];
		for (const month of months) { 
      		const sum = Math.round(await getWasm().Store.sum(`${year}:${month}`));
      		const pair: [string, number] = [month, sum];
			result.push(pair);
		}
		return result;
	}	
	
	async render() {
		const data = await this.money(2025, MONTHS_2025); 
		const data2 = await this.money(2026, MONTHS_2026); 
		this.tmpNodes.forEach(x => x.remove());
		this.list(2025, data);
		this.list(2026, data2);
		this.tmpNodes = [];
	}

	list(year: number, data: Array<[string, number]>) {
		const container = document.querySelector(`#list-row-${year}`)!;
		const result = [];
		for (const [month, sum] of data) {
      		const row = (document.querySelector("#template-row") as HTMLTemplateElement).content;
      		container.appendChild(this.fill(row.cloneNode(true) as HTMLElement, sum, year, month));
      		result.push(container.lastElementChild!);
		};
		return result;
	}

	fill(x: HTMLElement, sum: number, year: number, month: string) {
  		x.querySelector('#row-sum')!.textContent = String(sum);
  		x.querySelector('#row-link')!.textContent = getMonthBy(month, 'ru');
  		(x.querySelector('#row-link') as HTMLAnchorElement)!.href = `/?month=${month}&year=${year}`;
  		x.querySelector('#row-download')!.setAttribute('__id', `${year}:${month}`);
  		x.querySelector('#row-download')!.setAttribute('download', `${year}_${month}.csv`);
  		return x;
	}
}








