# Cargo Cartesi

Rust-specific toolkit for building DApps on Cartesi.

## Motivation

The Cartesi ecosystem on its own is self-sufficient. It, however, requires learning some domain knowledge. To smooth-out the learning curve, we developed a toolkit that allows Rust developer of any level to build DApps on Cartesi using a high-level interface.

## Requirements

* [Rust](https://rustup.rs/)
* [Docker](https://docs.docker.com/get-docker/)

## Installation

Clone this repository, then run

```bash
cargo build -p cargo-cartesi
```

And put directory `target/debug` in your `$PATH`.

## Usage

### Create new DApp

Use the following command to create hello world DApp and build from there.

```bash
cargo cartesi new my-dapp
```

Creates a new DApp in `my-dapp/` directory.

### Build binary

In the DApp directory run: 

```bash
cargo cartesi build
```

To build RISC-V specific binary. Cartesi machine is built on RISC-V architecture so that's the target we need.

### Create file-system

Cartesi machine uses ext2 file-system. To embed the binary in an image that the machine can mount, run:

```bash
cargo cartesi create-fs
```

### Build machine

To create Cartesi machine in an initial state, in the DApp directory run:

```bash
rm -rf pwd
mkdir pwd
cargo cartesi create-machine
```

The initial state is when the DApp runs on Cartesi machine until first yield. It will be stored in `pwd/machine` directory.

### Run (one-shot)

To run binary on a Cartesi machine without rollups, in the DApp directory run:

```bash
cargo cartesi run
```

This is an experimental feature, the app runs on Cartesi machine but is not able to yield or do any other operation using `MachineIo`.
