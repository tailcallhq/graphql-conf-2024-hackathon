#!/bin/bash

function format_json_table() {
    local json_file="$1"

    # Parse the JSON and flatten it
    local output=$(jq -r '
        ["Category", "Average", "Standard Deviation", "Max", "Stdev Percent"],
        ["Latency", .latency.avg, .latency.stdev, .latency.max, .latency.stdev_percent],
        ["Requests", .requests.avg, .requests.stdev, .requests.max, .requests.stdev_percent],
        ["Total Requests", .total_requests, "", "", ""],
        ["Memory Usage", .memory_usage, "", "", ""],
        ["Requests Per Second", .requests_per_second, "", "", ""],
        ["Transfer Per Second", .transfer_per_second, "", "", ""],
        ["Socket Errors (Connect)", .socket_errors.connect, "", "", ""],
        ["Socket Errors (Read)", .socket_errors.read, "", "", ""],
        ["Socket Errors (Write)", .socket_errors.write, "", "", ""],
        ["Socket Errors (Timeout)", .socket_errors.timeout, "", "", ""]
        | @tsv' "$json_file")

    # Print the header for the markdown table
    echo "| Category              | Average   | Standard Deviation  | Max        | Stdev Percent |"
    echo "|-----------------------|-----------|---------------------|------------|---------------|"

    # Use while loop to print each line from the jq output into markdown table format
    while IFS=$'\t' read -r category avg stdev max stdev_percent; do
        printf "| %-22s | %-9s | %-19s | %-10s | %-13s |\n" "$category" "$avg" "$stdev" "$max" "$stdev_percent"
    done <<< "$output"
}

# Usage
# Iterate through directories in /results
for dir in ./results/*/; do
    if [ -d "$dir" ]; then
        echo "Results for $dir"

        # Construct full path to bench_1.json
        json_file="$dir/bench_1.json"

        # Check if file exists
        if [ ! -f "$json_file" ]; then
            echo "Error: $json_file not found"
            continue
        fi

        format_json_table $json_file
    fi
done
