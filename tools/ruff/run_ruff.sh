#!/bin/bash

# The first argument is the path to the ruff binary
RUFF_BIN="$1"
shift

# Pass all remaining arguments to the ruff binary
exec "$RUFF_BIN" "$@"
