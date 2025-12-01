wasm.prod:
	cd wasm && ./build.sh --release && cd -

wasm.dev:
	cd wasm && ./build.sh && cd -

wasm.test:
	cd wasm && ./test.sh && cd -

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

.PHONY: b deploy

b: wasm.dev ts.dev 

deploy: pull.update wasm.test wasm.prod ts.install ts.prod version.create

