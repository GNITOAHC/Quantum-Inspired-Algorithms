#ifndef _ISINGMODEL_H_
#define _ISINGMODEL_H_

#include <vector>

#include "Nodes.h"

class IsingModel : public Nodes {
  public:
    // Constructor
    IsingModel(const Jxx& jxx);

    // Manipulator
    double annealing(const double& init, const double& tau);
};

#endif
