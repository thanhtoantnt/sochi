Sochi - Running Smart Contract Bug-Finding Tools
============================================================
# Prerequisites

## Rust

We use Rust stable for this project. It can be installed by the [rustup](https://rustup.rs/)
installer as follow:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
```

## Solc compiler

`solc` is used to compile Solidity smart contracts to JSON. You can compile `solc`
as follows. To work with Github actions, we currently use the latest version of `solc`.

```
        sudo add-apt-repository ppa:ethereum/ethereum
        sudo apt-get update
        sudo apt-get install solc
```
## Slither

## Mythril

## Solc-select
