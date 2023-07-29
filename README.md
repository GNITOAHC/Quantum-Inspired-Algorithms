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
# Usage: target/debug/fujitsu [-J <J>] [-Gamma <Gamma>] [-L <L>] [-H <H>] [--use-random] [--debug-output] [--without-cycle] [--help]
# J : Default strength for all pairs of x_i & x_j
# Gamma: Gamma will be calculated to default strength between layers
# L : Side length of this triangular lattice
# H : Height of this triangular lattice
# use-random : Use random strength between nodes
# debug-output : Log the nodes info and contents of output.json
# without-cycle : Generate triangular lattice without cycle
# help: List the available arguments

$ cargo run -- -J 1 -Gamma 1 -H 2 -L 3 --debug-output
```

## Calculation Concepts

Hamiltonian function:

$$
H_{eff} = K \sum_{<i, j>, n} s_{i, n}s_{j, n}  - K' \sum_{i, n} s_{i, n} s_{i, n+1}
$$

- In Ising models, the energy function (evaluation function) is determined by the interaction between spins that can take binary values {+1, -1}.
- The evaluation function is in a stable condition at its minimum.
- Instead of {+1, -1}, a binary value of {0, 1} expressed as a second-order polynomial
  is called a QUBO (Quadratic Unconstrained Binary Optimization)

$$
\sum K \sigma_i \sigma_j \rightarrow \sum I x_i x_j ~~~
x = \frac{\sigma + 1}{2} \in \{0, 1\}
$$

From $k(2x_i - 1)(2x_j - 1)$, we get $4kx_ix_j - 2kx_i - 2kx_j + k$.

## API Usage

Post request or get result from Fujitsu API. (File path `./api/`) Please check if there exists a `.env` file in `./api` and `FUJITSU_API_KEY` is set.

```dosini
# /api/.env
FUJITSU_API_KEY=<YOUR_API_KEY>
```

```shell
# Usage: sh compile [-hlspd] [job_id]
# -h | --help   Show api.sh usage
# -l | --list   List all jobs
# -s | --status Show status of current job (Must provide job_id)
# -g | --get    Save the status of current to ./target/Gamma<value>/<metadata>.json
# -p | --post   Post the question and json inside ./target/input.json to Fujitsu api and retrieve job id
# -d | --delete Delete the job (Must provide job_id)

sh api.sh --list # List all jobs
```

> **IMPORTANT**  
> Please run from `/api` directory

## Config Usage

Analysis the results retrieve from Fujitsu API. (File path `./config/`)

```shell
# Generate the data file for analysis.
sh config.sh ../target/Gamma0.0/Strength1.0_Lattice18_18_1_Time600.json

# Plot the data from specific datafile. (Run the above command to generate the datafile)
sh config.sh ../target/Gamma0.0/1.0_18_18_1.txt
```

> **IMPORTANT**  
> Please run from `/config` directory
