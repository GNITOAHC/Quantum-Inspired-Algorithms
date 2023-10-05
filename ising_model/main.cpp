#include <cmath>
#include <iostream>
#include <vector>

#include "IsingModel.h"
#include "Nodes.h"

/*
 * function -> camalCase
 * variable -> snake_case
 * class -> PascalCase
 * constant -> ALL_CAPS
 * global variable -> ALL_CAPS
 */

// Define global constants
const double E = std::exp(1.0);
const int INIT_TEMP = 10;

inline double loge (double x) { return std::log(x) / std::log(E); }

void printUsage (const char *prog_name) { std::cout << "Usage: " << prog_name << " J Gamma Length Height" << std::endl; }

bool isValidJxx(const Jxx& jxx);                                                   // Check if the given Jxx is valid
std::tuple<bool, Direction> prune(const int&, const int&, const int&, const int&); // Prune the lattice

// Parse input from stdin and return a tuple of Jxx and lattice
std::tuple<Jxx, std::map<std::pair<int, int>, std::pair<Direction, double> >, int> parseInput();

// ./main < 1.in
int main (int argc, char *argv[]) {
    const auto [jxx, lattice_config, tau] = parseInput();

    IsingModel ising_model(jxx);
    ising_model.configNodes(lattice_config);

    std::cout << "Parameter length squared: " << ising_model.getOrderParameterLengthSquared() << std::endl;
    std::cout << "Hamiltonian energy: " << ising_model.getHamiltonianEnergy() << std::endl;
    std::cout << "Hamiltonian energy: " << ising_model.annealing(INIT_TEMP, tau) << std::endl;
    ising_model.printConfigurations(0);

    return 0;
}

// Parse input from stdin and return a tuple of Jxx and lattice
std::tuple<Jxx, std::map<std::pair<int, int>, std::pair<Direction, double> >, int> parseInput () {
    double j, gamma, tau;
    int length, height;
    std::cin >> j >> gamma >> length >> height >> tau;
    Jxx jxx = { j, gamma, length, height, gamma == 0 ? 0.0 : (-0.5) * loge(tanh(gamma)) };
    try {
        isValidJxx(jxx);
    } catch (std::invalid_argument& e) {
        std::cout << e.what() << std::endl;
        exit(-1);
    } catch (...) {}

    int count = 0;
    std::cin >> count;
    std::map<std::pair<int, int>, std::pair<Direction, double> > lattice;
    for (int i = 0; i < count; ++i) {
        double co;
        int po1, po2;
        std::cin >> co >> po1 >> po2;
        auto [pruned, direction] = prune(po1, po2, length, height);
        if (pruned) continue;
        else { lattice[po1 < po2 ? std::make_pair(po1, po2) : std::make_pair(po2, po1)] = std::make_pair(direction, co); }
    }
    return { jxx, lattice, tau };
}

// Check if the given Jxx is valid
bool isValidJxx (const Jxx& jxx) {
    // Check if jxx.j, jxx.gamma, jxx.l, jxx.h are smaller than 0
    if (jxx.j < 0.0 || jxx.gamma < 0.0 || jxx.l < 0 || jxx.h < 0) {
        throw std::invalid_argument("All arguments must be positive");
        return false;
    }

    // Check if jxx.l is valid
    if (jxx.l % 3 != 0) {
        throw std::invalid_argument("Length must be a multiple of 3");
        return false;
    }

    // When gamma is 0, height must be 1
    if (jxx.gamma == 0.0 && jxx.h != 1) {
        throw std::invalid_argument("When gamma is 0, height must be 1");
        return false;
    }

    // When height is 1, gamma must be 0.0
    if (jxx.h == 1 && jxx.gamma != 0.0) {
        throw std::invalid_argument("When height is 1, gamma must be 0.0");
        return false;
    }

    return true;
}

// Check if the given lattice needs to be pruned
std::tuple<bool, Direction> prune (const int& a, const int& b, const int& length, const int& height) {
    // true for prune, false for not prune
    if (a == b) return { true, NIL };
    const int h = GET_H(a, length), i = GET_I(a, length), j = GET_J(a, length);
    const bool x[8] = { b == get_right(h, i, j, length),
                        b == get_bottom(h, i, j, length),
                        b == get_bottom_right(h, i, j, length),
                        b == get_layer_up(h, i, j, length, height),
                        b == get_left(a, length),
                        b == get_up_left(a, length),
                        b == get_up(a, length),
                        b == get_layer_down(a, length, length) };
    for (int i = 0; i < 8; ++i)
        if (x[i]) return { false, static_cast<Direction>(i) };
    return { true, NIL };
}
