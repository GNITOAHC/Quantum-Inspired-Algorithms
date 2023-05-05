# Quantum-inspired Algorithm 

## Rust installation 

```shell
$ brew install rustup-init && rustup-init   # MacOS
> winget install -e --id Rustlang.Rustup    # Windows
```

To verify the installation

```shell
$ rustc --version
```

## Compile & Run

```shell
$ cargo run # Compile & run with default options
```

To build manually and run

```shell
$ cargo build # Build
$ ./target/debug/jujitsu # Running on MacOS
$ . \target\debug\fujitsu.exe # Running on Windows
```

## Options

```shell
# Usage: target/debug/fujitsu [-J <J>] [-JL <JL>] [-L <L>] [-H <H>] [--use-random] [--debug-output]
# J : Default strength for all pairs of x_i & x_j
# JL: Default strength for all pairs of x_i & x_j but for layer between layer
# L : Side length of this triangular lattice
# H : Height of this triangular lattice
# use-random : Use random strength between nodes
# debug-output : Log the nodes info and contents of output.json

$ cargo run -- -J 1 -JL 2 -H 1 -L 6
```
