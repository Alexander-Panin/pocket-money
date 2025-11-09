const fs = require('node:fs');

function log(a,b, filename) {
	console.log(a === b ? "Err" : "Ok", filename)
}

function build(filename) {
	const content = fs.readFileSync(`../static/${filename}`, 'utf8');
	const newContent = content.replace(/\.(css|js)\?t=\d+/gm, `.$1?t=${+Date.now()}`);
	log(content, newContent, filename);
	fs.writeFileSync(`../static/${filename}`, newContent);
}

function build_wasm() {
	const content = fs.readFileSync(`../static/script/pkg/wasm.js`, 'utf8');
	const regexp = /new URL\(\'wasm_bg\.wasm(\?t=\d+)?\'/
	const newContent = content.replace(regexp, `new URL('wasm_bg.wasm?t=${+Date.now()}'`);
	log(content, newContent, "wasm.js");
	fs.writeFileSync(`../static/script/pkg/wasm.js`, newContent);
}

['index.html', 'calendar.html'].forEach(build)

build_wasm()
