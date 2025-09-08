let __popup_handler = (ev: Event) => {};

export const handler = (popupId: string) => (event: Event) => {
	const {__id: rowId = null, __action: action} = get_attrs(event.target as Element);
	switch (action) {
		case 'list/row':
			link(popupId, popup_handler(rowId));
			show(popupId);
			return;
		case 'popup/close':
			unlink(popupId);
			hide(popupId);
	}
}

function unlink(popupId: string) {
	document
		.querySelector(popupId)
	   ?.removeEventListener('click', __popup_handler);
}

function link(popupId: string, handler: (e: Event) => void) {
	const x = document.querySelector(popupId);
	x?.removeEventListener('click', __popup_handler);
	__popup_handler = handler;
	x?.addEventListener('click', __popup_handler);
}

const show = (popupId: string) => 
	{ (document.querySelector(popupId) as HTMLElement).hidden = false; }

const hide = (popupId: string) => 
	{ (document.querySelector(popupId) as HTMLElement).hidden = true; }

const popup_handler = (rowId: string | null) => (event: Event) => {
	const {__action: action} = get_attrs(event.target as Element);
	switch (action) {
		case 'popup/update': 
			event.stopPropagation();
			console.log("popup/update XXXX, rowId:", rowId);
	}
}

function get_attrs(elem: Element) {
	const map = {} as Record<string, string>;
	for (let x of elem.attributes) {
		map[x.name] = x.value;
	}
	return map;
}