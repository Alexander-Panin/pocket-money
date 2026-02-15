import getWasm from "../common/wasm";

export class DataHash {
	index: string;
	prefix: PrefixHash;

	constructor() {
		this.index = ""; 
		this.prefix = new PrefixHash("");
		this.build(undefined);
	}

	async build(index: string | undefined) {
		this.index = index ?? await this.read_fast();
		this.prefix = new PrefixHash(this.index);
	}

	async rebuild() {
		const index = await	this.read_slow();
		this.write_fast(index);
		await this.build(index);
	}

	async read_slow(): Promise<string> {
		const keys = ["2025:december"]; /* todo */
		const result = [];
		for (const key of keys) {
			const days = await getWasm().Store.all(key);
			result.push(days.map(format_record(key)).join("¥"));
		}
		return result.join("¥");
	}

	async read_fast(): Promise<string> {
		return this.read_slow(); /* todo */
	}

	async write_fast(index: string) {
		/* todo */
	}
}

export function record(index: string, i: number): string {
	const f = index.lastIndexOf("¥", i);
	const l = index.indexOf("¥", i);
	return index.slice(f+1, l);
}

class PrefixHash {

	prefix: any; // PrefixHash

	constructor(index: string) {
		const [xs, n] = str2ab(index);
		this.prefix = getWasm().PrefixHash.new(n);
		this.prefix.build(xs);
	}

	find(key: string) {
		const newKey = key.slice(0,30);
		const [ys] = str2ab(newKey);
		return this.prefix.find(ys);
	}
}

const format_record = (key: string) => (d: Day): string => {
	const round = (price: number) => Math.round(price * 10) / 10;
	const c = (comment: string) => comment === "" ? " " : ` ${comment} `;
	return `[${d.tag}] €${round(d.price)} ${d.date}:${key} ${c(d.comment)}`;
}

function ab2str(buf: Uint16Array): String {
    return String.fromCharCode.apply(null, buf as unknown as number[]);
}

function str2ab(str: string): [Uint16Array, number] {
    const n = str.length;
    const buf = new ArrayBuffer(n*2); // two bytes for each char
    const bufView = new Uint16Array(buf);
    for (let i=0; i !== n; i++) {
    	bufView[i] = str.charCodeAt(i);
    }
    return [bufView, n];
}