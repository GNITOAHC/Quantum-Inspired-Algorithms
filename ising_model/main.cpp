#include <cmath>
#include <iostream>
#include <vector>

#include "IsingModel.h"
#include "Nodes.h"

using namespace std;

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

bool isValidJxx(Jxx& jxx); // Check if the given Jxx is valid

// ./main J Gamma Length Height tau

int main (int argc, char *argv[]) {

    Jxx jxx;

    if (argc != 6) {
        printUsage(argv[0]);
        return -1;
    }

    // Parse command line arguments (J, Gamma, Length, Height, tau)
    jxx.j = atof(argv[1]);     // J
    jxx.gamma = atof(argv[2]); // Gamma
    jxx.l = atoi(argv[3]);     // Length
    jxx.h = atoi(argv[4]);     // Height

    jxx.jl = (-0.5) * loge(tanh(jxx.gamma));

    // Check if the given Jxx is valid
    try {
        isValidJxx(jxx);
    } catch (std::invalid_argument& e) {
        std::cout << e.what() << std::endl;
        return -1;
    } catch (...) {}

    printf("jl = %f\n", jxx.jl);

    IsingModel ising_model(jxx);

    std::cout << "Parameter length squared: " << ising_model.getOrderParameterLengthSquared() << std::endl;
    std::cout << "Hamiltonian energy: " << ising_model.getHamiltonianEnergy() << std::endl;
    std::cout << "Hamiltonian energy: " << ising_model.annealing(INIT_TEMP, atoi(argv[5])) << endl;

    return 0;
}

// Check if the given Jxx is valid
bool isValidJxx (Jxx& jxx) {
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

    // Override jxx.jl if gamma is 0.0
    if (jxx.gamma == 0.0) { jxx.jl = 0.0; }

    return true;
}
