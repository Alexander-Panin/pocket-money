import getWasm from "../common/wasm";
import * as download from "./download";

export class Listener {
	constructor() { }

	handler = (event: Event) => {
		const node = getWasm().target(event.target as Element);
		if (!node?.attributes) return;
		const action = node.attributes.getNamedItem('__action')?.value;
		const id = node.attributes.getNamedItem('__id')?.value ?? "";
		if (action === "ns/skip") return;
		event.preventDefault();
		switch (action) {
			case 'download':
				this.download(node, id);
				return;
			
		}
	}

	async download(link: HTMLAnchorElement, ns: string) {
		const hash = await download.payload(ns);
		download
			.setLinksAttrs(link, hash)
			.click();
		window.URL.revokeObjectURL(hash);
	}
}

