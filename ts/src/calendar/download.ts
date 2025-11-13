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
	return content;
	// const encoder = new TextEncoder()
	// const content = await csv(ns);
	// const blob = new Blob([encoder.encode(content)], {type: "text/plain;charset=UTF-8"});
	// return window.URL.createObjectURL(blob);
}

export function setLinksAttrs(link: HTMLAnchorElement, hash: string): HTMLAnchorElement {
	let attr = link.attributes.getNamedItem('__action')!;
	attr.value = "ns/skip";
	link.attributes.setNamedItem(attr);
	// link.href = hash;
	// link.setAttribute('href', 'data:text/plain;charset=UTF-8,' + encodeURIComponent(hash));
	link.setAttribute('href', 'data:text/plain;charset=UTF-8;base64,' + btoa(unescape(encodeURIComponent(hash))));
	return link;
}

