# Plot the data

import matplotlib.pyplot as plt
import numpy as np
import sys


num_bins = 50


def plot_data(c6, order_p_length):
    n, x = np.histogram(c6, num_bins, density=True)
    bin_centers = 0.5 * (x[1:] + x[:-1])
    plt.plot(bin_centers, n)

    n, x = np.histogram(order_p_length, num_bins, density=True)
    bin_centers = 0.5 * (x[1:] + x[:-1])
    plt.plot(bin_centers, n)

    plt.show()


def main():
    file_path = sys.argv[1]
    content = np.loadtxt(file_path)
    plot_data(content[:, 0], content[:, 1])
    pass


if __name__ == "__main__":
    main()


"""
def main():
    data = np.loadtxt("c18_18_1.txt")
    data1 = data[:,0]
    data2 = data[:,1]

    n, x  = np.histogram(data1, num_bins, density=True)
    bin_centers = 0.5*(x[1:]+x[:-1])
    plt.plot(bin_centers, n)

    n, x  = np.histogram(data2, num_bins, density=True)
    bin_centers = 0.5*(x[1:]+x[:-1])
    plt.plot(bin_centers, n)

    plt.show()

"""
