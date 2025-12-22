import getWasm from "../../common/wasm";
import worker from "../../common/worker";

export default class Comment {
	model: Day
	row: Element

	constructor(model: Day, row: Element) {
		this.model = model;
		this.row = row;
		this.fill(this.model.comment);
	}

	action(event: Event) {
		const action = (event.target as Element).attributes.getNamedItem('__action')?.value;
		switch (action) {
			case 'comment':
				this.comment((event.target as HTMLTextAreaElement).value);
		}
	}

	fill(comment: string) { 
		(document.querySelector("#comment") as HTMLInputElement).value = comment ?? "";
		(document.querySelector("#comment") as HTMLInputElement).placeholder = comment ?? ""; 
	}

	async comment(comment: string) { 
		(document.querySelector("#comment") as HTMLInputElement).value = comment; 
		(this.row.querySelector('#row-comment') as HTMLElement).textContent = comment; 
		await worker("save_comment", {id: this.model.id, value: comment});
		await getWasm().save_comment_fast(this.model.id, comment);
		this.model.comment = comment;
	}
}