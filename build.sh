set -e

BEG='\n\033[0;34m'
END='\033[0m'

# get project name in Cargo.toml
PROJECT=$(awk -F '"' '/name/ {gsub(/-/, "_", $2); print $2}' Cargo.toml)

echo -e "${BEG}Installing wasm-bindgen${END}"
cargo install wasm-bindgen-cli --version "0.2.72"

echo -e "${BEG}Building project${END}"
cargo build --release --target wasm32-unknown-unknown

echo -e "${BEG}Building wasm${END}"

mkdir -p wasm

$HOME/.cargo/bin/wasm-bindgen \
        --no-typescript \
        --target web \
        --out-dir wasm/ \
        ./target/wasm32-unknown-unknown/release/${PROJECT}.wasm 


echo -e "${BEG}Running website${END}"
python3 -m http.server
