#!/bin/bash
# Program
#   This script is for Fujitsu Digital Annealer API

# API='X-Api-Key:<API_KEY>'
if [ -f .env ]; then # Check .env file
    FUJITSU_API_KEY=`cat .env | grep FUJITSU_API_KEY | cut -d '=' -f 2`
    if [ -z $FUJITSU_API_KEY ]; then # Check FUJITSU_API_KEY in .env file
        echo "Please set FUJITSU_API_KEY in .env file" && exit 1
        exit 1
    fi
else
    echo "Please set FUJITSU_API_KEY in .env file"
    exit 1
fi

API="X-Api-Key:$FUJITSU_API_KEY"
ACCEPT="Accept:application/json"
CONTENT_TYPE="Content-type:application/json"
BASE_URL="https://api.aispf.global.fujitsu.com"

# POST
# curl -H 'X-Api-Key:<API_KEY>' -H 'Accept: application/json' -H 'Content-type: application/json' -X POST -d @<JSON_FILE> <BASE_URL>/da/v3/async/qubo/solve
# GET status
# curl -H 'X-Api-Key:<API_KEY>' -H 'Accept: application/json' -H 'Content-type: application/json' -X GET <BASE_URL>/da/v3/async/jobs/result/<JOB_ID>
# GET list
# curl -H 'X-Api-Key:<API_KEY>' -H 'Accept: application/json' -H 'Content-type: application/json' -X GET <BASE_URL>/da/v3/async/jobs
# DELETE
# curl -H 'X-Api-Key:<API_KEY>' -H 'Accept: application/json' -H 'Content-type: application/json' -X DELETE <BASE_URL>/da/v3/async/jobs/result/<JOB_ID>

USAGE="
Usage: sh api.sh [-hlspd] [job_id]\n
\n
Options:\n
    \t-h, --help      \tshow this help message and exit\n
    \t-l, --list      \tlist all jobs\n
    \t-s, --status    \tshow job status [jobs_id]\n
    \t-p, --post      \tregister qubo optimization jobs\n
    \t-d, --delete    \tdelete job [jobs_id]\n
"

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            echo $USAGE
            exit 1
            ;;
        -l|--list)
            cmd="list"
            shift
            ;;
        -s|--status)
            cmd="status"
            shift
            ;;
        -g|--get)
            cmd="get"
            shift
            ;;
        -p|--post)
            cmd="post"
            shift
            ;;
        -d|--delete)
            cmd="delete"
            shift
            ;;
        *)
            JOB_ID=$1
            shift
            ;;
    esac
done

# Check if job_id is set
check_job() {
    if [ -z $JOB_ID ]; then
        echo "job_id is required"
        exit 1
    fi
}

get_meta() {
    Gamma=`python3 ./get_meta.py Gamma`
    Strength=`python3 ./get_meta.py Strength`
    Height=`python3 ./get_meta.py Height`
    Side_length=`python3 ./get_meta.py Side_length`
    echo "Please ensure that this job_id is corresponding to the following parameters: "
    echo "Gamma: $Gamma\n Strength: $Strength\n Height: $Height\n Side_length: $Side_length\n"
    echo "[y/n]: " && read ans
    if [ $ans = "n" ]; then
        echo "Please set job_id again"
        exit 1
    fi
    METADATA="Gamma${Gamma}_Strength${Strength}_Lattice${Side_length}*${Side_length}*${Height}"
}

if [ $cmd = "list" ]; then
    curl -H $API -H $ACCEPT -H $CONTENT_TYPE -X GET $BASE_URL/da/v3/async/jobs | json_pp
elif [ $cmd = "status" ]; then
    check_job
    curl -H $API -H $ACCEPT -H $CONTENT_TYPE -X GET $BASE_URL/da/v3/async/jobs/result/$JOB_ID | json_pp
elif [ $cmd = "get" ]; then
    check_job && get_meta
    curl -H $API -H $ACCEPT -H $CONTENT_TYPE -X GET $BASE_URL/da/v3/async/jobs/result/$JOB_ID | json_pp > ../target/result_$METADATA.json
elif [ $cmd = "post" ]; then
    curl -H $API -H $ACCEPT -H $CONTENT_TYPE -X POST -d @../target/input.json $BASE_URL/da/v3/async/qubo/solve | json_pp
elif [ $cmd = "delete" ]; then
    check_job
    curl -H $API -H $ACCEPT -H $CONTENT_TYPE -X DELETE $BASE_URL/da/v3/async/jobs/result/$JOB_ID | json_pp
else
    echo "invalid command"
fi
