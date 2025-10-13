import * as tabs from "./tabs";

const onceMapId = new Set();

const NS = "2025:august"; // todo

function createModel(wasm: Wasm, id: string) {
	const model = Boolean(id) 
		? wasm.Day.fetch(id) 
		: wasm.Day.new_with_date(wasm.Store.stats(NS)?.last_date ?? new Date().getDate());
	if (!Boolean(id)) { onceMapId.add(model.id); }
	return model;
}

export class Popup {
	wasm: Wasm
	row: Element
	model: Day
	view: tabs.Money | tabs.Comment | tabs.Tag | tabs.Year | null

	constructor(wasm: Wasm, id: string, row: Element) {
		this.wasm = wasm;
		this.model = createModel(wasm, id);
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
		if (onceMapId.has(this.model.id)) {
			this.row.setAttribute('__id', this.model.id);
			onceMapId.delete(this.model.id);
			this.wasm.Store.append(NS, this.model);
		}
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
				this.view = new tabs.Tag(this.model, this.row, this.wasm.Store.tags(NS));
				return;
			case 'nav/year':
				this.tab('year');
				this.view = new tabs.Year(this.model);
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