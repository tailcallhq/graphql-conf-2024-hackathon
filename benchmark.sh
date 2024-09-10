#!/usr/bin/env bash

set -e

bench_test=$1

wrk -d 30 -t 4 -c 100 -s benches/${bench_test}.lua http://localhost:8000/graphql
