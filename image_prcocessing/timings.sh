#!/bin/bash


PAGE_NUMBER=0
PAGE_SIZE=100

URL="api/articles/${PAGE_NUMBER}/${PAGE_SIZE}"



start=`date +%s`
time curl http://localhost:2345/multithreaded/${URL} | jq > rust_multi.json
end=`date +%s`
runtime_rust_multi=$((end-start))



start=`date +%s`
time curl http://localhost:2345/singlethreaded/${URL} | jq > rust_single.json
end=`date +%s`
runtime_rust_single=$((end-start))


start=`date +%s`

time curl http://localhost:2323/${URL} | jq > java_ref.json
end=`date +%s`
runtime_java_ref_impl=$((end-start))



echo "============================================"
echo "============================================"
echo "rust multi        ${runtime_rust_multi}"
echo "rust single       ${runtime_rust_single}"
echo "java ref impl     ${runtime_java_ref_impl}"
echo "============================================"
echo "============================================"

