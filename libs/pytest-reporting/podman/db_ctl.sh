#!/bin/bash
set -e

# Ensure we are in the project root
if [ -n "$BUILD_WORKSPACE_DIRECTORY" ]; then
    cd "$BUILD_WORKSPACE_DIRECTORY"
fi

podman-compose -f libs/pytest-reporting/podman/docker-compose.yml "$@"
