#!/bin/zsh

curl -vvv -w "@curl-format.txt" -s   http://192.168.0.115:12120/api/v1/articles/0/3 > v1.json


curl -vvv -w "@curl-format.txt" -s   http://192.168.0.115:12120/api/v1/articles/0/3 > v2.json

