# Get the metadata of dataset for analysis

import sys


def main():
    file_path = sys.argv[1]
    action = sys.argv[2]
    if action == "FileType":
        print(get_file_type(file_path))
        return
    elif action == "Gamma":
        print(get_gamma(file_path))
        return
    elif action == "Metadata":
        print(get_metadata(file_path))
        return


# Json file formet ex: ../target/Gamma0.0/Strength1.0_Lattice18_18_1_Time600.json


def get_file_type(file_path):
    data: list[str] = file_path.split(".")
    return data[-1]


def get_gamma(file_path):
    gamma: str = file_path.split("/")[2].lstrip("Gamma")
    return gamma


def get_metadata(file_path):
    strength: str = file_path.split("/")[-1].split("_")[0].lstrip("Strength")
    length: str = file_path.split("/")[-1].split("_")[2]
    height: str = file_path.split("/")[-1].split("_")[3]
    return f"{strength}_{length}_{length}_{height}"


if __name__ == "__main__":
    main()
