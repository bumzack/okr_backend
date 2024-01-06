#!/bin/zsh

curl -vvv -w "@curl-format.txt" -s   http://localhost:2323/api/articles/0/2 -o /dev/null
