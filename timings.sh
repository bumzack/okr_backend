#!/bin/bash

IMPORT_URL=(    "http://localhost:2323/api/v1/articles/import/false"  "http://localhost:2323/api/v2/articles/import/false" )
SYSINFO_URL=(   "http://localhost:2323/api/v1/sysinfo"                "http://localhost:2323/api/v2/sysinfo"         )

ITERATIONS=10

WARMUPS=3

FILENAME=results.txt

if [ ! -f "$FILENAME" ]; then
  echo " author ;  language; framework ; version ; multi threaded ; comment ; slowest run ; fastest run ;  measurements excl slowest and fastest " >> $FILENAME
fi


LEN=${#IMPORT_URL[@]}

echo "==================================================================================================="
echo "===============    ${WARMUPS} warmup run    ======================================================="
echo "==================================================================================================="

for (( i=0; i<$LEN; i++ ))
do
    URL=${IMPORT_URL[i]}
    for (( iter=1; iter<=$WARMUPS; iter++ ))
    do
        DURATION=$(curl -w "@curl-format.txt" -s -o /dev/null -X POST  ${URL} )
        echo "warmup  #${iter} / ${WARMUPS} took ${DURATION} secs     ${URL}  "
    done
done
echo "==================================================================================================="
echo "===============     warmup done             ======================================================="
echo "==================================================================================================="




LEN=${#IMPORT_URL[@]}


for (( i=0; i<$LEN; i++ ))
do
    URL=${SYSINFO_URL[i]}
    SYSINFO=$(curl  -s  ${URL} | jq )
    # do whatever on "$i" here

    AUTHOR=$(echo "$SYSINFO" | jq '.author' | sed 's/\"//g' )
    LANGUAGE=$(echo "$SYSINFO" | jq '.language' | sed 's/\"//g' )
    FRAMEWORK=$(echo "$SYSINFO" | jq '.framework' | sed 's/\"//g' )
    MULTI=$(echo "$SYSINFO" | jq '.multithreaded' | sed 's/\"//g' )
    COMMENT=$(echo "$SYSINFO" | jq '.comment' | sed 's/\"//g' )
    VERSION=$(echo "$SYSINFO" | jq '.version' | sed 's/\"//g' )

    URL=${IMPORT_URL[i]}

    echo "==================================================================================================="
    echo "===============  ${ITERATIONS} test runs    ======================================================="
    echo "==================================================================================================="


    durations=()
    for (( iter=1; iter<=$ITERATIONS; iter++ ))
    do
        DURATION=$(curl -w "@curl-format.txt" -s -o  /dev/null -X POST  ${URL} )
        echo "run #${iter} / ${ITERATIONS} took ${DURATION} secs           ${URL} "
        durations+=( ${DURATION} )
    done

    IFS=$'\n' 
    sorted=($(sort -n <<<"${durations[*]}"))

    middle=( )

    MAX=$(($ITERATIONS - 1))
    for (( idx=1; idx<$MAX; idx++ ))
    do
          middle+=( ${sorted[idx]} )
    done

    FASTEST=${sorted[0]}
    SLOWEST=${sorted[idx]}
    measurements=$(IFS=';' ; echo "${middle[*]}")

    # echo "SLOWEST ${SLOWEST}     FASTEST ${FASTEST}   measurements   ${measurements}"



    echo "==================================================================================================="
    echo "===============   test runs done   ================================================================"
    echo "==================================================================================================="


    echo ""
    echo ""
    echo "=============== finished  =========================================================================="
    echo " author     ${AUTHOR} "
    echo " language   ${LANGUAGE} "
    echo " framework  ${FRAMEWORK} "
    echo " multi      ${MULTI} "
    echo " comment    ${COMMENT} "
    echo " version    ${VERSION} "
    echo " slowest    ${SLOWEST} "
    echo " fastest    ${FASTEST} "
    echo "==================================================================================================="

    echo " ${AUTHOR} ;  ${LANGUAGE}; ${FRAMEWORK} ; ${VERSION} ; ${MULTI} ; ${COMMENT} ; ${SLOWEST} ; ${FASTEST} ;  ${measurements} " >> $FILENAME

    echo ""
    echo ""
    echo ""
    echo ""
done