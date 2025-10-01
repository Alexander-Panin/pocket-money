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
