#!/bin/zsh

# PPM=$( curl http://192.168.0.115:2323/api/articles/0/1 | jq '.[0]  | .images |  map(select (.width == 64))  | .[0]  | .image ' | sed 's/\"//g' )


curl http://192.168.0.115:2323/api/articles/0/1 | jq '.[0]  | .images |  map(select (.width == 64))  | .[0] '  > image.json
FILENAME=$( cat image.json  | jq ' .filename  ' | sed 's/\"//g' )
PPM=$(cat image.json | jq '  .image ' | sed 's/\"//g' )
echo "filename   ${FILENAME}"
echo ${PPM} > ${FILENAME}.ppm
