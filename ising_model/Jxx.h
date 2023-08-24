#ifndef _TYPES_H_
#define _TYPES_H_

typedef struct {
    double j;     // J{i, j} of the node
    double gamma; // Gamma of the Hamiltonian
    int l;        // Side length of the triangular lattice
    int h;        // Height of the triangular lattice
    double jl;    // J_{i,j} of x_i, x_j, but for layer between layer
} Jxx;

#endif
