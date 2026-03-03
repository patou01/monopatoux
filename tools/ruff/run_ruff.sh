#!/bin/bash

# Convert the ruff binary path to an absolute path
RUFF_BIN="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
shift

# Change to the project root directory
cd "$BUILD_WORKSPACE_DIRECTORY"
echo "Current directory after cd: $(pwd)"

# Pass all remaining arguments to the ruff binary
exec "$RUFF_BIN" "$@"
