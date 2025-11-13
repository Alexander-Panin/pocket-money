import getWasm from "../common/wasm";

function one(x: Day, isNext: boolean) {
	return [
		isNext ? String(Math.round(x.date)) : "", 
		String(Math.round(x.price * 10) / 10),
		x.tag, 
		x.comment
	];
}

async function csv(ns: string): Promise<string> {
	const data = await getWasm().Store.select(ns);
	return ["date,price,tag,comment"]
		.concat(
			data.map((x: [boolean, Day]) => one(x[1],x[0]).join(","))
		).join("\n");
}

export async function payload(ns: string): Promise<string> {
	const content = await csv(ns);
	const blob = new Blob([content], {type: "data:text/csv;charset=UTF-8"});
	return window.URL.createObjectURL(blob);
}

export function setLinksAttrs(link: HTMLAnchorElement, hash: string): HTMLAnchorElement {
	let attr = link.attributes.getNamedItem('__action')!;
	attr.value = "ns/skip";
	link.attributes.setNamedItem(attr);
	link.href = hash;
	return link;
}

