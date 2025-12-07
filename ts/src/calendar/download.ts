import getWasm from "../common/wasm";

function one(x: Day, isNext: boolean) {
	return [
		isNext ? String(Math.round(x.date)) : "", 
		JSON.stringify(String(Math.round(x.price * 10) / 10).replace(".", ",")),
		JSON.stringify(x.tag), 
		JSON.stringify(x.comment), 
	];
}

type FirstRecord = [boolean, Day];
async function csv(ns: string): Promise<string> {
	const data = await getWasm().Store.select(ns, 0 /* asc */);
	return ["date,price,tag,comment"]
		.concat(
			data.map((x: FirstRecord) => 
				(x[0] ? ",,,\n" : "") + one(x[1], x[0]).join(",")
			)
		).join("\n");
}

export async function payload(ns: string): Promise<string> {
	const encoder = new TextEncoder()
	const content = await csv(ns);
	const blob = new Blob([encoder.encode(content)], {type: 'text/csv;charset=utf-8;'});
	return window.URL.createObjectURL(blob);
}

export function setLinksAttrs(link: HTMLAnchorElement, hash: string): HTMLAnchorElement {
	let attr = link.attributes.getNamedItem('__action')!;
	attr.value = "ns/skip";
	link.attributes.setNamedItem(attr);
	link.href = hash;
	return link;
}

