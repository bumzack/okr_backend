#!/bin/bash

IMPORT_URL=(  "http://localhost:2323/api/v1/articles/import"  "http://localhost:2323/api/v2/articles/import" )

LEN=${#IMPORT_URL[@]}

for (( i=0; i<$LEN; i++ ))
do
    URL=${IMPORT_URL[i]}
    curl -s  -X POST  ${URL}  -s    -d '{ "returnItems": false  }'  -H "Content-Type: application/json"| jq > test_business_requirements_devdata_actual.json
    DIFF=$(diff -Naur test_business_requirements_devdata_actual.json test_business_requirements_devdata_expected.json  )
    DIFF_LEN=${#DIFF}

    if [ "$DIFF_LEN" -gt "0" ]; then
        echo "===============  ACTUAL RESPONSE != EXPECTED RESPONSE =========================================="
        echo "===============  ${URL} "
        echo ""
        echo "diff between actual and expected"
        echo "$DIFF"
        echo "==============================================================================================="
    else
      echo "===============  OK    ${URL} "
    fi
done

echo "===============    finished                ======================================================="
