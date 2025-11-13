wasm.prod:
	cd wasm && ./build.sh --release && cd -

wasm.dev:
	cd wasm && ./build.sh && cd -

ts.install:
	cd ts && npm install && cd -

ts.prod:
	cd ts && npm run build:prod && cd -

ts.dev:
	cd ts && npm run build && cd -

version.create:
	cd cli && node build_html.js && cd -

pull.update:
	git checkout -- .
	git pull origin master

build: wasm.dev ts.dev version.create

deploy: pull.update wasm.prod ts.install ts.prod version.create


