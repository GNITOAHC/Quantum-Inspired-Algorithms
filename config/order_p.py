# Order parameter calculation and analysis

import numpy as np
import json
import math
import sys

import plot


class MatAnalyser(object):
    data_order_p_length: list[float] = []
    data_c6: list[float] = []

    def __init__(self, data_order_p_length, data_c6) -> None:
        self.data_order_p_length = data_order_p_length
        self.data_c6 = data_c6
        pass

    def plot(self):
        plot.plot_data(self.data_c6, self.data_order_p_length)
        # print("Plotting...")


# sys.argv[1]: The json file to be analysed from Fujitsu
# sys.argv[2]: Whether to output the data
# sys.argv[3]: Whether to plot the data
def main():
    metadata: dict[str, float] = split_filename(sys.argv[1])
    # print("meta: ", metadata)
    length_from_qubo_sol: int = 0  # Init the length from qubo solution
    with open(sys.argv[1], "r") as f:
        data = json.load(f)
        length_from_qubo_sol = len(data["qubo_solution"]["solutions"])
    # print("Length from qubo solution: ", length_from_qubo_sol)

    L: int = int(metadata["Length"])  # Lattice side length

    # Calculate order parameter
    # Create the data storage file

    output: bool = False
    if sys.argv[2] == "True" or sys.argv[2] == "true":
        output = True
    plot_data: bool = False
    if sys.argv[3] == "True" or sys.argv[3] == "true":
        plot_data = True

    analysis_data: list[list[float]] = get_order_parameter(
        L, length_from_qubo_sol, output
    )
    if plot_data:
        mat = MatAnalyser(analysis_data[0], analysis_data[1])
        mat.plot()


def get_order_parameter(L: int, length_of_qubo: int, output: bool) -> list[list[float]]:
    # L: Side length of the lattice
    # length_of_qubo: Length of the qubo solution
    # output_file: sys.argv[2]
    # data = np.empty((2, 0)).tolist()
    data: list[list[float]] = [[], []]
    for i in range(0, length_of_qubo):
        m_color_params: list[int] = [0, 0, 0]  # BLUE, BLACK, RED
        m_each_count: list[int] = [0, 0, 0]  # BLUE, BLACK, RED

        with open(sys.argv[1], "r") as f:
            content = json.load(f)["qubo_solution"]["solutions"][i]["configuration"]
            for index in content:
                idx: int = int(index)  # Convert to int
                idx %= L**2
                remainder: int = (math.floor(idx / L) + idx) % 3

                m_each_count[remainder] += 1
                if content[index] == True:
                    m_color_params[remainder] += 1
                else:
                    m_color_params[remainder] -= 1

            m_blue: float = m_color_params[0] / m_each_count[0]
            m_black: float = m_color_params[1] / m_each_count[1]
            m_red: float = m_color_params[2] / m_each_count[2]

            imag_pi = complex(0, 4 / 3 * math.pi)
            order_parameter = (
                m_blue
                + m_black * (math.e**imag_pi)
                + m_red * (math.e ** (-1 * imag_pi))
            ) / math.sqrt(3)
            order_p_6 = order_parameter**6
            if order_p_6 == 0:  # Check if delimeter is 0
                continue
            c6 = np.real(order_p_6) / np.abs(order_p_6)

            data[0].append(c6)
            data[1].append(
                np.real(order_parameter) ** 2 + np.imag(order_parameter) ** 2
            )
    if output:
        for i in range(0, len(data[0])):
            print(f"{data[0][i]}\t{data[1][i]}")
    return data


def split_filename(filename: str) -> dict[str, float]:
    # filename e.x.: ../target/Gamma0.0/Strength1.0_Lattice18_18_1_Time600.json
    args: list[str] = filename.split("/")
    dataset: dict[str, float] = {}
    dataset["Gamma"] = float(args[2].lstrip("Gamma"))
    dataset["Strength"] = float(args[3].split("_")[0].lstrip("Strength"))
    dataset["Length"] = float(args[3].split("_")[1].lstrip("Lattice"))
    dataset["Height"] = float(args[3].split("_")[3])
    return dataset


if __name__ == "__main__":
    main()
