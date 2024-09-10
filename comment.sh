#!/usr/bin/env bash

set -e

score=$1

markdown_content=$(cat <<EOF
## Hackathon Score Report

The score is: **$score**

---

More detailed stats can be found inside the ci run for Benchmark.

Thank you for your submission!
EOF
)

echo "$markdown_content" > body.md
