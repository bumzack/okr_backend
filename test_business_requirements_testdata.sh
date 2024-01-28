#!/bin/bash

IMPORT_URL=( "http://localhost:2325/api/v1/articles/import"  "http://localhost:2325/api/v2/articles/import"  )


LEN=${#IMPORT_URL[@]}

for (( i=0; i<$LEN; i++ ))
do
    URL=${IMPORT_URL[i]}
    curl -s  -X POST  ${URL} -d '{ "returnItems": true  }'  -H "Content-Type: application/json" | jq > testdata_actual.json
    DIFF=$(diff -Naur testdata_actual.json testdata_expected.json)
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
