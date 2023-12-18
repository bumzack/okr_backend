#!/bin/zsh


echo "========================================================="
echo "=========== Java Ref Impl ==============================="
echo "========================================================="

curl  -w "@curl-format.txt" -s  -o /dev/null  http://localhost:2323/api/articles



echo "========================================================="
echo "=========== Rust Warp single threaded   ================="
echo "========================================================="

curl  -w "@curl-format.txt" -s  -o /dev/null  http://localhost:2345/api/articles


echo "========================================================="
echo "=========== Rust Warp multi  threaded   ================="
echo "========================================================="

curl  -w "@curl-format.txt" -s  -o /dev/null  http://localhost:2346/api/articles




echo "========================================================="
echo "=========== Rust Warp rayon   ================="
echo "========================================================="

curl  -w "@curl-format.txt" -s  -o /dev/null  http://localhost:2347/api/articles



curl  -w "@curl-format.txt" -s  -o /dev/null  http://localhost:2323/api/articles/0/3


