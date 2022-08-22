# Tranferring one sol

Tranferring one sol using the system program in Solana

# Dependencies

Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

Solana

```
sh -c "$(curl -sSfL https://release.solana.com/v1.9.1/install)"
```

Yarn

```
npm install -g yarn
```

Anchor (install wsl in VSC)

```
npm i -g @project-serum/anchor-cli
cargo install --git https://github.com/project-serum/anchor --tag v0.24.2 anchor-cli --locked
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
```

# Commands
```
anchor run create
```
```
anchor run send
```
