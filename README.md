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

```
USAGE:
    fujitsu [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug-output     Output debug information
    -h, --help             Prints help information
    -u, --use-random       Use random strength for each node
    -V, --version          Prints version information
    -w, --without-cycle    Without cycle

OPTIONS:
    -g, --gamma-analysis <file-path>         Gamma analysis target file
    -G, --gamma <gamma>                      Gamma of the Hamiltonian
        --guidance-config <guidance-path>    Use guidance config
    -H, --height <h>                         Height of the triangular Lattice
    -J, --J <j>                              J_{i,j} of x_i, x_j
    -L, --length <l>                         Side length of the triangular Lattice
    -T, --time <t>                           Time limit of the Fujitsu request
```

Example: `cargo run -- --help`

### Gamma Analysis format

```rs
/*
* Config file structure
* {
*   "qubo_solution": {
*       "progress": [{"energy": -1.44, "time": 0.252}],
*       "result_status": true,
*       "solutions": [
*           {
*               "configuration": {
*                   "0": true,
*                   "1": false,
*                   ...
*               },
*               "energy": -144,
*               "frequency": 1
*           },
*           ...
*       ],
*       "timing": {"solve_time": "10840", "total_elapsed_time": "11024"},
*  },
*  "status": "Done"
* }
*/
```

Config index is the index of solutions, i.e. `json["qubo_solution][solutions]`

> c6 / order parameter / config index / layer / energy

```shell
$ head target/Gamma0.0/1.0_12_12_1.txt
-0.8606409648666089     0.43889120273959936     0       0       -441
0.157870698204492       0.27568759930275966     1       0       -441
-0.9896054083329263     0.42673577367454896     2       0       -441
-0.838877739417418      0.0067255927314236325   3       0       -441
0.9845361435785488      0.05374303916577969     4       0       -441
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

## Config Usage (_deprecated_)

Analysis the results retrieve from Fujitsu API. (File path `./config/`)

```shell
# Generate the data file for analysis.
sh config.sh ../target/Gamma0.0/Strength1.0_Lattice18_18_1_Time600.json

# Plot the data from specific datafile. (Run the above command to generate the datafile)
sh config.sh ../target/Gamma0.0/1.0_18_18_1.txt
```

> **IMPORTANT**  
> Please run from `/config` directory
