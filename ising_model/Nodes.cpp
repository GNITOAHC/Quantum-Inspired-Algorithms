#include "Nodes.h"
#include <cmath>
#include <complex>
#include <cstdarg>

// Constructors

Nodes::Nodes() : nodes(std::vector<Node>()) { return; }

Nodes::Nodes(const Jxx& jxx) : jxx(jxx) {
    const int length = jxx.l;
    const int height = jxx.h;
    const int length2 = length * length;

    // Create sublattice lambda getter
    auto get_sub_lattice = [&] (const int index) -> SubLattice {
        const int sub = ((index / length) + index) % 3;
        switch (sub) {
            case 0: return BLUE;
            case 1: return BLACK;
            case 2: return RED;
            default: std::cout << "Error: get_sub_lattice() -> sub = " << sub << std::endl; throw std::exception();
        }
    };

    // Create the vector of NODES
    for (int h = 0; h < height; ++h) {
        for (int i = 0; i < length; ++i) {
            for (int j = 0; j < length; ++j) {

                const int index = h * length2 + i * length + j;
                const int right = get_right(h, i, j, length);
                const int bottom = get_bottom(h, i, j, length);
                const int bottom_right = get_bottom_right(h, i, j, length);
                const int layer_up = get_layer_up(h, i, j, length, height);
                const SubLattice sub_lattice = get_sub_lattice(index);

                // Create the node
                Node node(index, right, bottom, bottom_right, layer_up, sub_lattice, jxx);

                // Push the node into the vector
                this->pushBack(node);
            }
        }
    }

    return;
}

Nodes::Nodes(const std::vector<Node>& nodes) : nodes(nodes) { return; }

// Manipulators

void Nodes::pushBack(const Node& node) { nodes.push_back(node); }

// Accessors

// Get the order parameter length squared
double Nodes::getOrderParameterLengthSquared() const {
    // First, define some constants
    const int length = jxx.l;
    const int height = jxx.h;
    const std::complex<double> image_pi(0.0, (4.0 / 3.0) * M_PI);
    const std::complex<double> math_e(M_E, 0.0);

    // Create a vector of vectors to store the color parameters and count of each color
    // m_color_params[layer][color] = count of blue, black, red
    std::vector<std::vector<double> > m_color_params(height, std::vector<double>(3, 0));

    const int each_color_count_per_layer_int = (length * length) / 3;
    const double each_color_count_per_layer = (double)each_color_count_per_layer_int;

    for (const auto& it : this->nodes) {
        const int layer = it.getIndex() / (length * length);
        switch (it.getSubLattice()) {
            case BLUE: m_color_params[layer][0] += 1; break;
            case BLACK: m_color_params[layer][1] += 1; break;
            case RED: m_color_params[layer][2] += 1; break;
            default: throw std::exception();
        }
    }

    const std::complex<double> math_e_pow = std::pow(math_e, image_pi);
    const std::complex<double> math_e_pow_inv = std::pow(math_e, -image_pi);

    double sum = 0.0;

    // Iterate over each layer
    for (int i = 0; i < height; ++i) {
        const std::complex<double> m_blue(m_color_params[i][0] / each_color_count_per_layer, 0.0);
        const std::complex<double> m_black(m_color_params[i][1] / each_color_count_per_layer, 0.0);
        const std::complex<double> m_red(m_color_params[i][2] / each_color_count_per_layer, 0.0);

        // Calculate the order parameter
        const std::complex<double> order_parameter = (m_blue + m_black * math_e_pow + m_red * math_e_pow_inv) / sqrt(3.0);

        // Calculate the order parameter length squared
        const double order_parameter_length_squared = std::pow(order_parameter.real(), 2.0) + std::pow(order_parameter.imag(), 2.0);

        // Add the order parameter length squared to the sum
        sum += order_parameter_length_squared;
    }

    return sum / (double)height;
}

// Get the Hamiltonian energy
double Nodes::getHamiltonianEnergy() const {
    double sum = 0.0;
    for (const auto& it : this->nodes) {

        const double current_spin = (double)it.getSpin();

        // Get the strength between the current node and the right node
        sum += it.getJRight() * current_spin * (double)this->nodes[it.getRight()].getSpin();
        // Get the strength between the current node and the bottom node
        sum += it.getJBottom() * current_spin * (double)this->nodes[it.getBottom()].getSpin();
        // Get the strength between the current node and the bottom right node
        sum += it.getJBtmRight() * current_spin * (double)this->nodes[it.getBtmRight()].getSpin();
        // Get the strength between the current node and the layer up node
        sum -= it.getJLayerUp() * current_spin * (double)this->nodes[it.getLayerUp()].getSpin();
    }
    return sum;
}

double Nodes::getHamiltonianDifference(const int count, ...) const {
    va_list valist;

    double sum_to_modify = 0.0;

    /*
     * Delta E = -2 * K * (S_i * S_j) + 2 * K' * (S_i * S_j')
     * S_j = j which live in the same layer as i
     * S_j' = j which live in the other layers
     * K = J_{i,j}
     * K' = J_{i,j'}
     * Delta E = 2 * ( -K * (S_i * S_j) + K' * (S_i * S_j') )
     */

    va_start(valist, count);
    for (int i = 0; i < count; ++i) {
        const int current_index = va_arg(valist, int);
        const double current_spin = (double)this->nodes[current_index].getSpin();
        // A node will have 8 neighbors

        // neighbors stores the indices of the neighbors
        const int neighbors[] = { this->nodes[current_index].getRight(), this->nodes[current_index].getBtmRight(),
                                  this->nodes[current_index].getBottom(), this->nodes[current_index].getLayerUp() };

        // Get the strength between the current node and the right node
        sum_to_modify -= this->nodes[current_index].getJRight() * current_spin * (double)this->nodes[neighbors[0]].getSpin();
        // Get the strength between the current node and the bottom right node
        sum_to_modify -= this->nodes[current_index].getJBtmRight() * current_spin * (double)this->nodes[neighbors[1]].getSpin();
        // Get the strength between the current node and the bottom node
        sum_to_modify -= this->nodes[current_index].getJBottom() * current_spin * (double)this->nodes[neighbors[2]].getSpin();
        // Get the strength between the current node and the layer up node
        sum_to_modify += this->nodes[current_index].getJLayerUp() * current_spin * (double)this->nodes[neighbors[3]].getSpin();

        // 4 indeices from the other nodes
        // indices stores the indices of the nodes to modify
        const int indices[] = { get_left(current_index, jxx.l), get_up_left(current_index, jxx.l), get_up(current_index, jxx.l),
                                get_layer_down(current_index, jxx.l, jxx.h) };

        // Get the energy from the left neighbor
        sum_to_modify -= (double)this->nodes[indices[0]].getJRight() * current_spin * (double)this->nodes[indices[0]].getSpin();
        // Get the energy from the up left neighbor
        sum_to_modify -= (double)this->nodes[indices[1]].getJBtmRight() * current_spin * (double)this->nodes[indices[1]].getSpin();
        // Get the energy from the up neighbor
        sum_to_modify -= (double)this->nodes[indices[2]].getJBottom() * current_spin * (double)this->nodes[indices[2]].getSpin();
        // Get the energy from the down layer neighbor
        sum_to_modify += (double)this->nodes[indices[3]].getJLayerUp() * current_spin * (double)this->nodes[indices[2]].getSpin();
    }
    va_end(valist);

    return 2 * sum_to_modify;
}

// Printers

// Print the lattice
void Nodes::printLattice(const Jxx& jxx) const {
    printf("%d layers in total\n---\n", jxx.h);

    // for (int i = 0; i < jxx.h; ++i) {}

    for (int j = 0; j < jxx.l; ++j) {
        for (int k = 0; k < jxx.l; ++k) {
            std::cout << this->nodes[j * jxx.l + k].getSubLattice();
            printf("\t");
        }
        std::cout << std::endl;
    }

    return;
}
