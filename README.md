Insight - a Smart Contract Comprehension Tool
============================================================

This tool aims to provide summaries, statistics, and various useful information
which can be used to understand the source code and structure of smart
contracts.

# Usage

## Show all options
- Run `./target/debug/insight -h` to show all options of the tool.

## Example
- `./target/debug/insight --print-call-graph examples/flipper.sol `

## Useful Options

- `--print-call-graph`: view call graph in the contract
- `--print-contract-summary`: view a list of functions of the contracts
- `--print-dot`: produce a dot file of the call graph
- `--print-png`: produce a png file of the call graph
- `--print-pdf`: produce a pdf file of the call graph

## Contract Summary
```
./insight tests/contracts/call_graph.sol --pcs

+ library Library
  [public] library_func

+ contract ContractA
  [public] my_func_a

+ contract ContractB
  [public] constructor
  [public] my_func_b
  [public] my_func_a
  [public] my_second_func_b

```

## Call graph
Call graph can be generated in a dot/png/pdf file using options. For example,
you can generate a call graph in a png file using the following command.

```
./insight tests/contracts/call_graph.sol --print-png
```

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

## Graphviz
`insight` uses `graphviz` to produce a dot file and a `.png` file for the option
`--print-dot` and `--print-png`, respectively. 

You can download `graphivz` using this [link](https://graphviz.org/download/).

## Compile `insight`

- Run `make` to build the tool.

- The running binary file is stored in `target/debug/` directory.

## Troubleshooting

We configured [llvm-rust](https://github.com/sbip-sg/llvm-rust) (`llvm-sys`, `inkwell`, `llutil`) and [rutil](https://github.com/sbip-sg/rutil) as
submodules of this repository. If you face any compilation errors related to
these libraries, please try the following commands to update them first.

```sh
cd insight
git submodule update --init --recursive
git submodule update --remote --merge

cd insight/llvm-rust
git submodule update --init --recursive
git submodule update --remote --merge
```
