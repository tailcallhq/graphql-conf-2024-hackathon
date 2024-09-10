#!/bin/bash

# Function to parse bench_1.out file
parse_bench_file() {
    local file_path="$1"

    # Parse latency statistics
    latency_avg=$(awk '/Latency/{print $2}' "$file_path")
    latency_stdev=$(awk '/Latency/{print $3}' "$file_path")
    latency_max=$(awk '/Latency/{print $4}' "$file_path")
    latency_stdev_percent=$(awk '/Latency/{print $5}' "$file_path")

    # Parse requests statistics
    req_per_sec_avg=$(awk '/Req\/Sec/{print $2}' "$file_path")
    req_per_sec_stdev=$(awk '/Req\/Sec/{print $3}' "$file_path")
    req_per_sec_max=$(awk '/Req\/Sec/{print $4}' "$file_path")
    req_per_sec_stdev_percent=$(awk '/Req\/Sec/{print $5}' "$file_path")

    # Parse total requests and memory usage
    total_requests=$(awk '/requests in/{print $1}' "$file_path")
    memory_usage=$(awk '/requests in/{print $5}' "$file_path")

    # Parse Requests/sec metric
    requests_sec=$(grep -i "Requests/sec" "$file_path" | awk '{print $NF}')

    # Parse Transfer/sec metric
    transfer_sec=$(grep -i "Transfer/sec" "$file_path" | awk '{print $NF}')

    # Check if Socket errors line exists
    if grep -q "Socket errors:" "$file_path"; then
        # Parse socket errors
        connect_errors=$(grep "Socket errors:" "$file_path" | cut -d',' -f1 | awk '{print $NF}')
        read_errors=$(grep "Socket errors:" "$file_path" | cut -d',' -f2 | awk '{print $NF}')
        write_errors=$(grep "Socket errors:" "$file_path" | cut -d',' -f3 | awk '{print $NF}')
        timeout_errors=$(grep "Socket errors:" "$file_path" | cut -d',' -f4 | awk '{print $NF}')

        socket_errors_json="\"socket_errors\": {
            \"connect\": $connect_errors,
            \"read\": $read_errors,
            \"write\": $write_errors,
            \"timeout\": $timeout_errors
        }"
    else
        # Default values if Socket errors line is not present
        socket_errors_json="\"socket_errors\": {
            \"connect\": 0,
            \"read\": 0,
            \"write\": 0,
            \"timeout\": 0
        }"
    fi

    echo "{
        \"latency\": {
            \"avg\": \"$latency_avg\",
            \"stdev\": \"$latency_stdev\",
            \"max\": \"$latency_max\",
            \"stdev_percent\": \"$latency_stdev_percent\"
        },
        \"requests\": {
            \"avg\": \"$req_per_sec_avg\",
            \"stdev\": \"$req_per_sec_stdev\",
            \"max\": \"$req_per_sec_max\",
            \"stdev_percent\": \"$req_per_sec_stdev_percent\"
        },
        \"total_requests\": \"$total_requests\",
        \"memory_usage\": \"$memory_usage\",
        \"requests_per_second\": \"$requests_sec\",
        \"transfer_per_second\": \"$transfer_sec\",
        $socket_errors_json
    }"
}

# Iterate through directories in /results
for dir in ./results/*/; do
    if [ -d "$dir" ]; then
        bench_file="$dir/bench_1.out"

        if [ -f "$bench_file" ]; then
            echo "Processing $bench_file:"

            # Get the directory path of bench_1.out
            output_dir=$(dirname "$bench_file")

            # Create the JSON file name
            json_file="$output_dir/bench_1.json"

            # Parse the file and save output to bench_1.json
            parse_bench_file "$bench_file" > "$json_file"

            echo "JSON saved to $json_file"
        else
            echo "Warning: $bench_file not found."
        fi
    fi
done