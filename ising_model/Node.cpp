#include "Node.h"
#include <iostream>

// Constructors

Node::Node()
    : index(0), right(0), bottom(0), btm_right(0), layer_up(0), spin(UP), sub_lattice(RED), j_right(1.0), j_bottom(1.0), j_btm_right(1.0),
      j_layer_up(1.0) {
    return;
}

Node::Node(int index, int right, int bottom, int btm_right, int layer_up, SubLattice sub_lattice, Jxx jxx)
    : index(index), right(right), bottom(bottom), btm_right(btm_right), layer_up(layer_up), sub_lattice(sub_lattice) {
    this->spin = UP;
    this->j_right = jxx.j;
    this->j_bottom = jxx.j;
    this->j_btm_right = jxx.j;
    this->j_layer_up = jxx.jl;

    return;
}

Node::Node(int index, int right, int bottom, int btm_right, int layer_up, bool spin, SubLattice sub_lattice, double j_right, double j_bottom,
           double j_btm_right, double j_layer_up)
    : index(index), right(right), bottom(bottom), btm_right(btm_right), layer_up(layer_up), spin(UP), sub_lattice(sub_lattice), j_right(j_right),
      j_bottom(j_bottom), j_btm_right(j_btm_right), j_layer_up(j_layer_up) {
    return;
}

// Getters

int Node::getIndex() const { return index; }

int Node::getRight() const { return right; }

int Node::getBottom() const { return bottom; }

int Node::getBtmRight() const { return btm_right; }

int Node::getLayerUp() const { return layer_up; }

Spin Node::getSpin() const { return spin; }

SubLattice Node::getSubLattice() const { return sub_lattice; }

double Node::getJRight() const { return j_right; }

double Node::getJBottom() const { return j_bottom; }

double Node::getJBtmRight() const { return j_btm_right; }

double Node::getJLayerUp() const { return j_layer_up; }

// Setters

void Node::setIndex(const int index) { this->index = index; }

void Node::setRight(const int right) { this->right = right; }

void Node::setBottom(const int bottom) { this->bottom = bottom; }

void Node::setBtmRight(const int btm_right) { this->btm_right = btm_right; }

void Node::setLayerUp(const int layer_up) { this->layer_up = layer_up; }

void Node::setSpin(const Spin spin) { this->spin = spin; }

void Node::setSubLattice(const SubLattice sub_lattice) { this->sub_lattice = sub_lattice; }

void Node::setJRight(const double j_right) { this->j_right = j_right; }

void Node::setJBottom(const double j_bottom) { this->j_bottom = j_bottom; }

void Node::setJBtmRight(const double j_btm_right) { this->j_btm_right = j_btm_right; }

void Node::setJLayerUp(const double j_layer_up) { this->j_layer_up = j_layer_up; }

// Printers

void Node::printNode() const {
    printf("========== Node %d ==========\n", index);
    printf("Right: %d\tBottom: %d\tBtmRight: %d\tLayerUp: %d\n", right, bottom, btm_right, layer_up);
    printf("Spin: %d\tSubLattice: %d\n", spin, sub_lattice);
    printf("JRight: %f\tJBottom: %f\tJBtmRight: %f\tJLayerUp: %f\n", j_right, j_bottom, j_btm_right, j_layer_up);
    printf("\n");
    return;
}
