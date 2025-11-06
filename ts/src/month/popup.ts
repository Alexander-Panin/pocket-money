import getWasm from "../common/wasm";
import * as tabs from "./tabs";

const newIds: Set<string> = new Set();

async function fetch(ns: string, id: string) {
	const shouldCreate = !Boolean(id);
	const model = shouldCreate ? getWasm().Day.new() : await getWasm().Day.fetch(id);
	if (shouldCreate) { newIds.add(model.id); }
	return model;
}

async function appendIf(ns: string, model: Day) {
	if (newIds.delete(model.id)) {  
		const date = (await getWasm().Store.stats(ns))?.last_date;
		await getWasm().Store.append(ns, model.id);
		await getWasm().save_date(model.id, String(date ?? new Date().getDate()));
	}
}

export class Popup {
	ns: string
	row: Element
	model: Day
	view: tabs.Money | tabs.Comment | tabs.Tag | tabs.Year | null

	constructor(id: string, row: Element, ns: string) {
		this.ns = ns;
		this.row = row;
		this.view = null;
		this.link();
		this.model = getWasm().Day.empty();
		fetch(ns, id).then(m => {
			this.model = m;
			this.show();
		});
	}

	destroy() {
		this.hide();
		this.unlink();
	}

	link() {
		const x = document.querySelector("#container-popup");
		x?.addEventListener('click', this.handler);
		x?.addEventListener('input', this.handler);
	}

	unlink() {
		const x = document.querySelector("#container-popup");
		x?.removeEventListener('click', this.handler);
		x?.removeEventListener('input', this.handler);
	}

	show() {
		this.tab('money'); // default tab
		this.view = new tabs.Money(this.model, this.row);
	}

	hide() { (document.querySelector("#container-popup") as HTMLElement).hidden = true; }

	handler = (event: Event) => {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		if (action !== 'nav/close') { event.stopImmediatePropagation(); }
		action?.startsWith('nav/') ? this.handleNav(action) : this.handleChildren(event);	 
	}

	handleChildren(event: Event) {
		this.view?.action(event);
		this.row.setAttribute('__id', this.model.id);
		appendIf(this.ns, this.model);
	}

	handleNav(action: string) {
		switch (action) {
			case 'nav/comment':
				this.tab('comment');
				this.view = new tabs.Comment(this.model, this.row);
				return;
			case 'nav/money':
				this.tab('money');
				this.view = new tabs.Money(this.model, this.row);
				return;
			case 'nav/tag':
				this.tab('tag');
				this.view = new tabs.Tag(this.model, this.row, this.ns);
				return;
			case 'nav/year':
				this.tab('year');
				this.view = new tabs.Year(this.model, this.ns);
				return;
		}
	}

	tab(page: string) {
		const popup = document.querySelector("#container-popup")!;
		const template = (document.querySelector(`#template-${page}`) as HTMLTemplateElement).content;
		const container = popup.querySelector('#container-popup-main')!;
		container.replaceChildren(template.cloneNode(true));
		(document.querySelector("#container-popup") as HTMLElement).hidden = false; 
	}
}