# pocket-money

uninstall
1. cd ~
2. rm -rf pocket_money
3. cargo uninstall wasm-pack
4. rustup self uninstall
5. rm -rf ~/.nvm

install 
1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. curl https://drager.github.io/wasm-pack/installer/init.sh -sSf | bash
3. curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
4. cd ~
5. git clone --depth=1 https://github.com:Alexander-Panin/pocket-money.git
6. cd pocket_money
7. cd wasm && ./build.sh --release && cd -
8. cd ts && npm install && npm run build:prod && cd -
9. cd server && cargo run --release 80 &

ubuntu
apt-get update 
apt install git
apt install vim
apt-get install libc6-dev (if not exists) 

cmd:prod
cd server && RUST_LOG=access_log=info cargo run --release 80 2>&1 | tee -a /var/log/pocket-money/access.log &

logrotate:
/var/log/pocket-money/access.log { 
    daily 
    missingok 
    notifempty 
    maxsize 100M 
    rotate 8 
} 
logrotate -d /etc/logrotate.conf # to check it 

pitfalls:

1. Rust String values +60kb in release build (better to use JsValue)
2. moved Rust values from JS produce "null pass to rust"  
3. serde json around 20-30kb in prod build (better not to plan to stringify vectors)
4. ts listener: better do not couple with dom (react like) due to additional state
5. esbuild with flag --platform=neutral to use ts bundle like a lib
6. html template/class namespaces naming
7.  css sticky element (stick to parent)
7a. no external deps (ts 10kb)
8.  rust data layer: list(append) and model(save, fetch)
8a. localStorage data format (e.g. csv and how to save vector) 
9.  ts ctor/dtor helps with subscribing and rendering 
9a. problems with state - e.g. current row for popup
10. rust file server (static) and plus some rest api
11. manifist.json
12. breaks big css file in small ones (and problems with import url)
13. fast rendering without wasm
14. JsValue default may be not what excepted
15. Direct and invert operation keeps together (route ns and params)
