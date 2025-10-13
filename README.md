# pocket-money

uninstall
1. cd ~
2. rm -rf project_name
3. cargo uninstall wasm-pack
4. rustup self uninstall

install 
1. cd ~
2. git checkout project_name
3. cd project_name	
3. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
4. cargo install wasm-pack
5. cd wasm && chmod +x build.sh && ./build.sh --release && cd -
6. cd ts && npm install && npm run build:prod && cd -
7. cd server && cargo run --release


pitfalls:

1. Rust String values +60kb in release build (better to use JsValue)
2. moved Rust values from JS produce "null pass to rust"  
3. serde json around 20-30kb in prod build (better not to plan to stringify vectors)
4. ts listener: better do not couple with dom (react like) due to additional state
5. esbuild with flag --platform=neutral to use ts bundle like a lib
6. html template/class namespaces naming
7. css sticky element (stick to parent)
8. rust data layer: list(append) and model(save, fetch)
9. ts ctor/dtor helps with subscribing and rendering 
10. rust server file server (static) and plus some rest api
11. manifist.json
12. breaks big css file in small ones (and problems with import url)
13. fast rendering without wasm
14. JsValue default may be not what excepted
