#!/usr/bin/env bash

set -e

TAILCALL_LOG_LEVEL=error TC_TRACKER=false tailcall start schema.graphql
