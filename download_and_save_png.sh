#!/bin/zsh

curl  http://localhost:2323/api/articles | jq > java_ref.json
BASE64=$(curl  http://localhost:2323/api/articles | jq '.[0]' |  jq ".images" | jq '.[0]'  |  jq ".image")
BASE642=$(echo "$BASE64"  | sed "s/[\"]//g")
echo "$BASE642"  |  base64 --decode  > java_ref.png
identify java_ref.png


curl  http://localhost:2345/api/articles | jq > rust_single.json
BASE64=$(curl  http://localhost:2345/api/articles | jq '.[0]' |  jq ".images" | jq '.[0]'  |  jq ".image")
BASE642=$(echo "$BASE64"  | sed "s/[\"]//g")
echo "$BASE642"  |  base64 --decode  > rust_single.png
identify rust_single.png


curl  http://localhost:2346/api/articles | jq > rust_multi.json
BASE64=$(curl  http://localhost:2346/api/articles | jq '.[0]' |  jq ".images" | jq '.[0]'  |  jq ".image")
BASE642=$(echo "$BASE64"  | sed "s/[\"]//g")
echo "$BASE642"  |  base64 --decode  > rust_mutli.png
identify rust_mutli.png

curl  http://localhost:2346/api/articles | jq > rust_rayon.json
BASE64=$(curl  http://localhost:2347/api/articles | jq '.[0]' |  jq ".images" | jq '.[0]'  |  jq ".image")
BASE642=$(echo "$BASE64"  | sed "s/[\"]//g")
echo "$BASE642"  |  base64 --decode  > rust_rayon.png
identify rust_rayon.png


