#!/bin/zsh

BASE64=$(curl  http://localhost:2323/api/articles | jq '.[0]' |  jq ".images" | jq '.[0]'  |  jq ".image")
BASE642=$(echo "$BASE64"  | sed "s/[\"]//g")
echo "$BASE642"  |  base64 --decode  > bla.png

identify bla.png
