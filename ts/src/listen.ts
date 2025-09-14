type Day = {
    date: number;
    price: number;
    tag: string,
    comment: string,
};

let __popup_handler = (ev: Event) => {};

function target(node: Element | null): Element | null {
	while (node?.attributes && !get_attrs(node.attributes).__action) { 
		node = node.parentNode as Element; 
	} 
	return node;
} 

export const handler = (popupId: string, storageById: Function) => (event: Event) => {
	const node = target(event.target as Element);
	if (!node?.attributes) return;
	console.assert(Boolean(node?.attributes), "not found node");
	const {__id: rowId = null, __action: action} = get_attrs(node!.attributes);
	switch (action) {
		case 'list/row':
			link(popupId, popup_handler(rowId, storageById(rowId ?? "0"))); // todo handle rowId when null
			show(popupId, storageById(rowId ?? "0"));
			focus(node as HTMLElement);
			return;
		case 'popup/close':
			unlink(popupId);
			hide(popupId);
	}
}

function unlink(popupId: string) {
	const x = document.querySelector(popupId);
	x?.removeEventListener('click', __popup_handler);
	x?.removeEventListener('input', __popup_handler);
}

function link(popupId: string, handler: (e: Event) => void) {
	const x = document.querySelector(popupId);
	x?.removeEventListener('click', __popup_handler);
	x?.removeEventListener('input', __popup_handler);
	__popup_handler = handler;
	x?.addEventListener('click', __popup_handler);
	x?.addEventListener('input', __popup_handler);
}

const focus = (node: HTMLElement) => 
	{ node.focus(); node.scrollIntoView({ behavior: "smooth", block: "center" }); }

const show = (popupId: string, day: Day) => { 
	const value = String(Math.round(day.price * 100) / 100);
	(document.querySelector("#popup-input") as HTMLInputElement).value = value; 
	(document.querySelector(popupId) as HTMLElement).hidden = false; 
}

const hide = (popupId: string) => 
	{ (document.querySelector(popupId) as HTMLElement).hidden = true; }

const popup_handler = (rowId: string | null, day: Day) => (event: Event) => {
	const {__action: action} = get_attrs((event.target as Element).attributes);
	switch (action) {
		case 'popup/update': 
			event.stopPropagation();
			console.log("popup/update XXXX, rowId:", rowId);
			return;
		case 'popup/tab-money':
			console.log("XXXX", action, rowId);
			return; 
		case 'popup/slider-scale':
			sliderScale(event);
			return;
		case 'popup/slider-main':
			sliderMain(event, rowId as string);
			return;
	}
}

function sliderScale(event: Event) {
	const scale = parseInt((event.target as HTMLInputElement).value);
	const [min, max] = [Math.round(5*scale/10), Math.round(1.5**scale+32)];
	document.querySelector('#popup-slider-msg')!.textContent = `${min}–${max}`;
	(document.querySelector('#popup-slider-main') as HTMLInputElement).min = String(min*10);
	(document.querySelector('#popup-slider-main') as HTMLInputElement).max = String(max*10);
}

function sliderMain(event: Event, rowId: string) {
	const value = parseInt((event.target as HTMLInputElement).value);
	const x = String(value / 10);
	(document.querySelector("#popup-input") as HTMLInputElement).value = x;
	(document
		.querySelectorAll('.row')[parseInt(rowId)]!
	    .querySelector('#money') as HTMLElement).textContent = String(x); 
}

export function render(storageAll: Function) {
	document
		.querySelector("#list")!
		.appendChild(
			(document.querySelector("#template-list") as HTMLTemplateElement).content
		);
	document
		.querySelector("#popup")!
		.appendChild(
			(document.querySelector("#template-popup") as HTMLTemplateElement).content
		);
	render_row(storageAll);
}

function render_row(storageAll: Function) {
	const container = document.querySelector("#row")!;
	const template = (document.querySelector("#template-row") as HTMLTemplateElement).content;
	storageAll().forEach((d: Day, i: number) => {
		const tmp = template.cloneNode(true) as HTMLElement;
		tmp.querySelector("div")!.setAttribute('__id', String(i));
		tmp.querySelector("#money")!.textContent = String(Math.floor(d.price));
		tmp.querySelector("#money2")!.textContent = String(d.price);
		tmp.querySelector("#tag")!.textContent = d.tag;
		tmp.querySelector("#comment")!.textContent = d.comment;
		container.appendChild(tmp);
	});
}



function get_attrs(attributes: NamedNodeMap) {
	const map = {} as Record<string, string>;
	for (let x of attributes) {
		map[x.name] = x.value;
	}
	return map;
}