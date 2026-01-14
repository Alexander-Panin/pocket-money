### pocket-money

# uninstall
1. cd ~
2. rm -rf pocket_money
3. cargo uninstall wasm-pack
3a. cargo uninstall cargo-make
4. rustup self uninstall
5. rm -rf ~/.nvm

# install 
1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. curl https://drager.github.io/wasm-pack/installer/init.sh -sSf | bash
3. curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
4. git clone --depth=1 ...
5. make deploy

# run:
cd server && RUST_LOG=access_log=info cargo run --release 2>&1 | tee -a /var/log/pocket-money/access.log &

# ubuntu
apt-get update 
apt install git
apt install vim
apt-get install libc6-dev (if not exists)
apt-get install libssl-dev 
apt install pkg-config
apt install silversearcher-ag
apt install xclip 

# log rotate
/var/log/pocket-money/access.log { 
    daily 
    missingok 
    notifempty 
    maxsize 100M 
    rotate 8 
} 
logrotate -d /etc/logrotate.conf # to check it 

# vds
sudo adduser v_kuuo
sudo usermod -aG sudo v_kuuo
openssl rand -base64 32
/etc/ssh/sshd_config
  Port 2288
  PasswordAuthentication no
  PubkeyAuthentication yes
  sudo systemctl restart ssh
apt update -y && apt install ufw -y
ufw default deny incoming
ufw default allow outgoing
ufw allow 2288/tcp
ufw allow 1194/udp   
ufw allow 80/tcp    
ufw allow 443/tcp   
ufw enable
ufw status verbose

# utils
clean up browser storage:

var dir = await navigator.storage.getDirectory();
for await (var [name, handle] of dir) {
    console.log(name, handle);
    dir.removeEntry(name, {recursive: true});
} 