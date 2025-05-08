#!/bin/bash
set -e

# Build the release binary
cargo build --release

# Create build directory
mkdir -p build

# Copy the release binary
cp target/release/server-monitor-api build/

# Copy static directory
cp -r static build/

# Copy Dockerfile and docker-compose.yml
cp Dockerfile build/
cp docker-compose.yml build/

# (Optional) Zip the build directory
cd build
zip -r ../deploy-package.zip .
cd ..