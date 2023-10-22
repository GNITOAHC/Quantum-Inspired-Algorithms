#include "IsingModel.h"

#include <cmath>
#include <functional>
#include <random>

// Constructor

IsingModel::IsingModel(const Jxx& jxx) : Nodes(jxx) { return; }

// Manipulator

// Randomly execute the given function with probability rand
bool randomExec (const double rand, const std::function<void()> func) {
    // Random number generator
    std::mt19937 generator(std::random_device {}());
    std::uniform_real_distribution<double> dis(0.0, 1.0);

    if (dis(generator) < rand) {
        func();
        return true;
    }
    return false;
}

// Anneal the nodes given initial temperature and tau
double IsingModel::annealing(const double& init, const double& tau) {
    const double T0 = init;
    for (int i = 0; i < tau; ++i) {
        const double T = T0 * (1 - ((double)i / tau));
        for (auto& node : this->nodes) {
            // Calculate the PI_accept
            const double delta_E = this->getHamiltonianDifference(1, node.getIndex());
            const double PI_accept = std::min(1.0, std::exp(-delta_E / T));

            // Flip the node with probability PI_accept
            randomExec(PI_accept, [&] () { node.flip(); });
        }
    }

    return this->getHamiltonianEnergy();
}

void IsingModel::printConfigurations(const int& group) const {
    printf("Group index spin\n");
    for (const auto& node : this->nodes) {
        std::cout << group << " " << node.getIndex() << " " << node.getSpin() << std::endl;
    }
    std::cout << std::endl;
    return;
}

void IsingModel::printLattice() const {
    printf("index index energy\n");
    for (const auto& node : this->nodes) {
        std::cout << node.getIndex() << " " << node.getRight() << " " << node.getJRight() << std::endl;
        std::cout << node.getIndex() << " " << node.getBottom() << " " << node.getJBottom() << std::endl;
        std::cout << node.getIndex() << " " << node.getBtmRight() << " " << node.getJBtmRight() << std::endl;
        std::cout << node.getIndex() << " " << node.getLayerUp() << " " << node.getJLayerUp() << std::endl;
    }
}
