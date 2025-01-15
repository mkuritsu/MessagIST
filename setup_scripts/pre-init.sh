sudo apt update
sudo apt install -y openssl libssl-dev sqlcipher libsqlcipher-dev rustup pkg-config gcc g++ iptables-persistent
rustup default stable
cargo build
