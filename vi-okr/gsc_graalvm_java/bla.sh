#!/bin/zsh

BASE64=$(curl  http://localhost:2323/api/articles | jq '.[0]' |  jq ".images" | jq '.[0]'  |  jq ".image")
BASE642=$(echo "$BASE64"  | sed "s/[\"]//g")
echo "$BASE642"  |  base64 --decode  > bla.png

identify bla.png



curl http://192.168.0.115:2323/api/articles/0/1 | jq '.[0]  | .images |  map(select (.width == 256))  | .[0] '  > image.json
FILENAME=$( cat image.json  | jq ' .filename  ' | sed 's/\"//g' )
PPM=$(cat image.json | jq '  .image ' | sed 's/\"//g' )
echo "filename   ${FILENAME}"
echo ${PPM} > ${FILENAME}.ppm





curl http://192.168.0.115:2323/api/articles/0/10 | jq   > res.json


