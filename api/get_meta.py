import json
import sys

if __name__ == "__main__":
    data = json.load(open("../target/metadata.json"))
    arg = sys.argv[1]
    print(data[arg])
