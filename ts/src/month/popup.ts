import getWasm from "./wasm";
import * as tabs from "./tabs";

function fetch(id: string, ns: string) {
	const date = getWasm().Store.stats(ns)?.last_date ?? new Date().getDate();
	return Boolean(id) 
		? getWasm().Day.fetch(id) 
		: getWasm().Day.new_with_date(date);
}


export class Popup {
	ns: string
	row: Element
	model: Day
	view: tabs.Money | tabs.Comment | tabs.Tag | tabs.Year | null

	constructor(id: string, row: Element, ns: string) {
		this.ns = ns;
		this.model = fetch(id, ns);
		this.row = row;
		this.view = null;
		this.link();
		this.show();
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
		getWasm().Store.append(this.ns, this.model);
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
				this.view = new tabs.Tag(this.model, this.row, getWasm().Store.tags(this.ns));
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