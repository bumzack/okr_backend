#!/bin/bash

IMPORT_URL=(    "http://localhost:2323/api/v1/articles/import"  "http://localhost:2323/api/v2/articles/import" )
SYSINFO_URL=(   "http://localhost:2323/api/v1/sysinfo"          "http://localhost:2323/api/v2/sysinfo"         )

ITERATIONS=10

FILENAME=results.txt

LEN=${#IMPORT_URL[@]}

echo "LEN ${LEN}"

for (( i=0; i<$LEN; i++ ))
do
    URL=${SYSINFO_URL[i]}
    echo "sysinfo   url ${URL}"
    SYSINFO=$(curl  -s  ${URL} | jq )
    # do whatever on "$i" here

    AUTHOR=$(echo "$SYSINFO" | jq '.author' | sed 's/\"//g' )
    LANGUAGE=$(echo "$SYSINFO" | jq '.language' | sed 's/\"//g' )
    FRAMEWORK=$(echo "$SYSINFO" | jq '.framework' | sed 's/\"//g' )
    MULTI=$(echo "$SYSINFO" | jq '.multithreaded' | sed 's/\"//g' )
    COMMENT=$(echo "$SYSINFO" | jq '.comment' | sed 's/\"//g' )
    VERSION=$(echo "$SYSINFO" | jq '.version' | sed 's/\"//g' )

    # echo $SYSINFO

    echo "author            ${AUTHOR}"
    echo "language          ${LANGUAGE}"
    echo "framework          ${FRAMEWORK}"
    echo "multithreaded     ${MULTI}"
    echo "comment           ${FRAMEWORK}"

    URL=${IMPORT_URL[i]}

    durations=()
    for (( iter=1; iter<=$ITERATIONS; iter++ ))
    do
        DURATION=$(curl -w "@curl-format.txt" -s -o /dev/null -X POST  ${URL} )
        echo "run #${iter} took ${DURATION}secs"
        durations+=( ${DURATION} )
    done

    IFS=$'\n' 
    sorted=($(sort -n <<<"${durations[*]}"))

    # echo "============================================ "
    # echo "durations in array "
    # for dur in "${sorted[@]}"
    # do
    #     echo "duration ${dur}"
    # done
    # echo "============================================ "

    middle8=( )

    MAX=$(($ITERATIONS - 1))
    echo "MAX   $MAX "
    for (( idx=1; idx<$MAX; idx++ ))
    do
          middle8+=( ${sorted[idx]} )
    done
    echo "============================================ "
    
    # echo "middle8   ${middle8[*]}"

    # echo "============================================ "
    # echo "durations in middle8 "
    # for dur in "${middle8[@]}"
    # do
    #     echo "duration middle8 ${dur}"
    # done
    # echo "============================================ "

    FASTEST=${sorted[0]}
    SLOWEST=${sorted[idx]}
    measurements=$(IFS=';' ; echo "${middle8[*]}")

    # echo "SLOWEST ${SLOWEST}     FASTEST ${FASTEST}   measurements   ${measurements}"

    echo " ${AUTHOR} ;  ${LANGUAGE}; ${FRAMEWORK} ; ${VERSION} ; ${MULTI} ; ${COMMENT} ; ${SLOWEST} ; ${FASTEST} ;  ${measurements} " >> $FILENAME
done