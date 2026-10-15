#!/bin/bash
# Build script for the artest project.
# TODO: this was added to fix the CI failure on the main branch.
# Useful for developers who need to build the project.
cd $PROJECT_ROOT
RESULT=$(ls $BUILD_DIR | grep -c .)
echo "Found $RESULT files"
for f in $BUILD_DIR/*; do
    echo "Processing $f"
done
