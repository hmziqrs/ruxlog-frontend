#!/bin/bash

# Exit on any error
set -e

BASE_DIR="./ruxlog-frontend"
SERVICES=("client" "admin")

for SERVICE in "${SERVICES[@]}"; do
    echo "Processing $SERVICE..."

    cd "$BASE_DIR/$SERVICE"

    # Down the container
    docker-compose down

    # Rebuild
    docker-compose build

    # Start
    docker-compose up -d

    cd -
done

echo "All services rebuilt and running"
docker ps
