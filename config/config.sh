#!/bin/bash
# Program
#   This script is for analyzing the result of fujitsu

# python3 ./get_meta.pa [KEY: key]
FILE_PATH=$1
FILE_TYPE=`python3 ./get_meta.py $FILE_PATH FileType`

# python3 ./plot.py [PATH_TO_TXT: txt file]
if [ $FILE_TYPE = "txt" ]; then
    echo "Plotting..."
    python3 ./plot.py $FILE_PATH
    exit 1
elif [ $FILE_TYPE = "json" ]; then
    Gamma=`python3 ./get_meta.py $FILE_PATH Gamma`
    METADATA=`python3 ./get_meta.py $FILE_PATH Metadata` # Get Metadata string, ex: 1_9_9_1 (strength_sideLength_sideLength_height)

    # python3 ./order_p.py [PATH_TO_JSON: json file] [OUTPUT_RESULT: bool] [PLOT_RESULT: bool]
    echo "Loading..."
    python3 ./order_p.py $FILE_PATH true false > ../target/Gamma${Gamma}/${METADATA}.txt

    read -p "Do you want to plot the result? (y/n) " yn
    if [ $yn = "y" ]; then
        python3 ./plot.py ../target/Gamma${Gamma}/${METADATA}.txt
    fi
else
    echo "File type error!"
    exit 1
fi

