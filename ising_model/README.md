# Ising Model

## Prerequisite

First, build the project via `make`.

## Running the script

First, create your input file with format `J Gamma Length Height Tau`.

- J - Default strength between two qubit
- Gamma - Will be translate to `jl` i.e. the default strength between layers via `(-0.5) * loge(tanh(gamma))`
- Length - The length of the triangular lattice
- Height - The height of the triangular lattice
- Tau - The $\tau$ when decreasing the _T0_ temperature

Next, customize your default values with format `coefficient polynomial1 polynomial2` with amount of `n`

```txt
J Gamma Length Height Tau
n
co po1 po2
... with amount of n
co po1 po2
```

e.x.

```txt
1.0 0.0 9 1 10000
2
5 1 2
4 2 3
```

Last, run the script with the given input file

```shell
$ cat sample.in # See what's inside `sample.in`
1.0 0.0 9 1 10000
0

$ ./main < sample.in # Run with `sample.in`
Hamiltonian energy: 243
Hamiltonian energy: -81
Parameter length squared: 0.641975
```
