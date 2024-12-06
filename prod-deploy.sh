#!/bin/bash
# deploy-prod.sh

# Exit on any error
set -e

BASE_DIR="./"
SERVICES=("client" "admin")
export PROJECT="rux"

echo "Starting production deployment process..."

# Create network if it doesn't exist
docker network create ${PROJECT}_network 2>/dev/null || echo "Network already exists"

for SERVICE in "${SERVICES[@]}"; do
    echo "Processing $SERVICE..."

    cd "$BASE_DIR/$SERVICE"

    # Down the container and remove volumes
    echo "Stopping $SERVICE containers and cleaning up..."
    docker compose --env-file .env.prod -f docker-compose.prod.yml down -v

    # Remove old images
    echo "Removing old images for $SERVICE..."
    docker image prune -f --filter "label=com.docker.compose.project=${PROJECT}"

    # Clean build cache
    echo "Cleaning build cache..."
    docker builder prune -f

    # Rebuild with no cache
    echo "Rebuilding $SERVICE..."
    docker compose --env-file .env.prod -f docker-compose.prod.yml build --no-cache

    # Start
    echo "Starting $SERVICE..."
    docker compose --env-file .env.prod -f docker-compose.prod.yml up -d

    cd -
done

echo "All services rebuilt and running"
docker ps
