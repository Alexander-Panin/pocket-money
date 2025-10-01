#!/bin/bash

wasm-pack build ${1:---dev} --target web --out-dir ../static/dist/pkg