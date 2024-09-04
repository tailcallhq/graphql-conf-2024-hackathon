bench_test=$1

wrk -d 30 -t 4 -c 100 -s benches/wrk${bench_test}.lua http://localhost:8000/graphql
