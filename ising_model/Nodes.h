#ifndef _NODES_H_
#define _NODES_H_

#include "Direction.h"
#include "Jxx.h"
#include "Node.h"

#include <iostream>
#include <map>
#include <vector>

class Nodes {
  protected:
    std::vector<Node> nodes;
    Jxx jxx;

  public:
    // Constructors
    Nodes();
    Nodes(const Jxx& jxx);
    Nodes(const std::vector<Node>& nodes);

    // Manipulators
    void configNodes(const std::map<std::pair<int, int>, std::pair<Direction, double> >&); // Modify the nodes according to the given map
    void pushBack(const Node& node);

    // Accessors
    double getOrderParameterLengthSquared() const;               // Get the order parameter length squared
    double getHamiltonianEnergy() const;                         // Get the Hamiltonian energy
    double getHamiltonianDifference(const int count, ...) const; // Get the Hamiltonian difference given the indices to flip and the spin

    // Printers
    void printLattice(const Jxx& jxx) const; // Print the lattice
};

// Inline helper functions for Nodes::Nodes(const Jxx& jxx) const;
inline int get_right (const int h, const int i, const int j, const int length) {
    const int _j = (j + 1) % length;
    return h * length * length + i * length + _j;
}
inline int get_bottom (const int h, const int i, const int j, const int length) {
    const int _i = (i + 1) % length;
    return h * length * length + _i * length + j;
}
inline int get_bottom_right (const int h, const int i, const int j, const int length) {
    const int _i = (i + 1) % length;
    const int _j = (j + 1) % length;
    return h * length * length + _i * length + _j;
}
inline int get_layer_up (const int h, const int i, const int j, const int length, const int height) {
    const int _h = (h + 1) % height;
    return _h * length * length + i * length + j;
}

#define GET_I(index, length) ((index / length) % length)
#define GET_J(index, length) (index % length)
#define GET_H(index, length) (index / (length * length))
// Inline helper functions for Nodes::getHamiltonianDifference(const int count, ...) const;
// i for index, l for length, h for height
inline int get_left (const int i, const int l) {
    const int _j = (GET_J(i, l) - 1) >= 0 ? (GET_J(i, l) - 1) : (GET_J(i, l) - 1) + l;
    return GET_H(i, l) * l * l + GET_I(i, l) * l + _j;
}
inline int get_up_left (const int i, const int l) {
    const int _i = (GET_I(i, l) - 1) >= 0 ? GET_I(i, l) - 1 : GET_I(i, l) - 1 + l;
    const int _j = (GET_J(i, l) - 1) >= 0 ? GET_J(i, l) - 1 : GET_J(i, l) - 1 + l;
    return GET_H(i, l) * l * l + _i * l + _j;
}
inline int get_up (const int i, const int l) {
    const int _i = (GET_I(i, l) - 1) >= 0 ? GET_I(i, l) - 1 : GET_I(i, l) - 1 + l;
    return GET_H(i, l) * l * l + _i * l + GET_J(i, l);
}
inline int get_layer_down (const int i, const int l, const int h) {
    const int _h = (GET_H(i, l) - 1) >= 0 ? GET_H(i, l) - 1 : GET_H(i, l) - 1 + h;
    return _h * l * l + GET_I(i, l) * l + GET_J(i, l);
}

#endif
