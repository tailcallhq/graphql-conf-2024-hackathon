#!/bin/bash

# Loop through subfolders
for dir in ./results/*/; do
    if [ -d "$dir" ]; then
        echo "Checking $dir"

        # Construct full path to bench_1.json
        json_file="$dir/bench_1.json"

        # Check if file exists
        if [ ! -f "$json_file" ]; then
            echo "Error: $json_file not found"
            continue
        fi

        # Read the JSON file using jq
        socket_errors=$(jq -r '.socket_errors | {connect: .connect, read: .read, write: .write, timeout: .timeout}' "$json_file")


        # Check if any socket error value is greater than zero
        if [[ $socket_errors =~ ([0-9]+) ]]; then
            connect=$(echo $socket_errors | grep -oP '"connect":\s*\K[0-9]+' | head -n 1 || echo 0)
            read=$(echo $socket_errors | grep -oP '"read":\s*\K[0-9]+' | head -n 1 || echo 0)
            write=$(echo $socket_errors | grep -oP '"write":\s*\K[0-9]+' | head -n 1 || echo 0)
            timeout=$(echo $socket_errors | grep -oP '"timeout":\s*\K[0-9]+' | head -n 1 || echo 0)
            if [[ $connect > 0 || $read > 0 || $write > 0 || $timeout > 0 ]]; then
                echo "Error: Socket errors detected in $dir"
                exit 1
            fi
        fi
    fi
done
