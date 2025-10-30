#!/bin/bash

echo "=== Filler Project Setup and Test ==="
echo ""

# Step 1: Clean Docker
echo "Step 1: Cleaning Docker..."
docker system prune -a -f
echo ""

# Step 2: Build Docker image
echo "Step 2: Building Docker image (this may take 10-15 minutes)..."
cd /Users/alijasim/filler/docker_image
docker build -t filler .
echo ""

# Step 3: Show how to access
echo "=== Docker image built successfully! ==="
echo ""
echo "Step 3: Run the container:"
echo "  docker run -v \"\$(pwd)/solution\":/filler/solution -it filler"
echo ""
echo "Step 4: Inside the container, compile:"
echo "  cd solution"
echo "  cargo build --release"
echo ""
echo "Step 5: Test your player:"
echo "  cd /filler"
echo "  ./linux_game_engine -f maps/map00 -p1 solution/target/release/player -p2 linux_robots/wall_e"
echo ""
echo "To pass the audit, win 4/5 times against:"
echo "  - wall_e on map00"
echo "  - h2_d2 on map01"
echo "  - bender on map02"
echo ""
