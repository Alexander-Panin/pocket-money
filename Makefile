wasm:
	cd wasm && ./build.sh --release && cd -

wasm.dev:
	cd wasm && ./build.sh && cd -

ts.install:
	cd ts && npm install && cd -

ts:
	cd ts && npm run build:prod && cd -

ts.dev:
	cd ts && npm run build && cd -

version:
	cd cli && node build_html.js && cd -

pull:
	git checkout -- .
	git pull origin master

build: wasm.dev ts.dev version

deploy: pull wasm ts.install ts version


