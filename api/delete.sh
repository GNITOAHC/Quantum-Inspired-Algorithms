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

# Get all jobs
curl -H $API -H $ACCEPT -H $CONTENT_TYPE -X GET $BASE_URL/da/v3/async/jobs | json_pp > delete_jobs.txt
cat delete_jobs.txt | grep '"job_id" : *' | cut -d ':' -f 2 | cut -d '"' -f 2 > job_list.txt

while read JOB_ID; do
    curl -H $API -H $ACCEPT -H $CONTENT_TYPE -X DELETE $BASE_URL/da/v3/async/jobs/result/$JOB_ID > /dev/null
done < job_list.txt

rm -f delete_jobs.txt job_list.txt
