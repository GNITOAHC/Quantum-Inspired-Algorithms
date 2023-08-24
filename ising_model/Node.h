#ifndef _NODE_H_
#define _NODE_H_

#include "Jxx.h"
#include "Spin.h"
#include "SubLattice.h"

class Node {

  private:
    int index;              // Index of the node
    int right;              // 2D index of the right node
    int bottom;             // 2D index of the bottom node
    int btm_right;          // 2D index of the bottom right Node
    int layer_up;           // 3D index of the layer up Node
    Spin spin;              // Spin of the node, true: up, false: down
    SubLattice sub_lattice; // Sub-lattice color of the node
    double j_right;         // J_{i,j} of current node and right node
    double j_bottom;        // J_{i,j} of current node and bottom node
    double j_btm_right;     // J_{i,j} of current node and bottom right
    double j_layer_up;      // J_{i,j} of current node and layer up

  public:
    // Constructors
    Node();
    Node(int index, int right, int bottom, int btm_right, int layer_up, SubLattice sub_lattice, Jxx jxx);
    Node(int index, int right, int bottom, int btm_right, int layer_up, bool spin, SubLattice sub_lattice, double j_right, double j_bottom,
         double j_btm_right, double j_layer_up);

    // Getters
    int getIndex() const;
    int getRight() const;
    int getBottom() const;
    int getBtmRight() const;
    int getLayerUp() const;
    Spin getSpin() const;
    SubLattice getSubLattice() const;
    double getJRight() const;
    double getJBottom() const;
    double getJBtmRight() const;
    double getJLayerUp() const;

    // Setters
    void setIndex(const int index);
    void setRight(const int right);
    void setBottom(const int bottom);
    void setBtmRight(const int btm_right);
    void setLayerUp(const int layer_up);
    void setSpin(const Spin spin);
    void setSubLattice(const SubLattice sub_lattice);
    void setJRight(const double j_right);
    void setJBottom(const double j_bottom);
    void setJBtmRight(const double j_btm_right);
    void setJLayerUp(const double j_layer_up);

    // Printers
    void printNode() const;
};

#endif
