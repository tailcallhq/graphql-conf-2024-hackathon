#!/usr/bin/env bash

set -e

npm install -g @tailcallhq/tailcall

# Get the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Set the path to the schema file relative to the script location
SCHEMA_FILE="${SCRIPT_DIR}/tailcall.graphql"

echo "Starting the server"

# Start Tailcall
TAILCALL_LOG_LEVEL=error TC_TRACKER=false tailcall start "${SCHEMA_FILE}"
